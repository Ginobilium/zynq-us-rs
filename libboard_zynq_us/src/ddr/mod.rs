//! DDR initialization and configuration
use libm::ceilf;
use libregister::{RegisterRW, RegisterW};

use super::slcr::{common::Unlocked, crf_apb};
use super::{print, println};
pub mod phy;
pub mod regs;
pub mod spd;

#[cfg(feature = "target_zcu111")]
// Micron MTA4ATF51264HZ-2G6E1, DDR4, max data rate 2666 MHz
// But US+ only supports up to 2133 MHz for a single-rank DIMM
pub const DDR_FREQ: u32 = 2_133_333_333;

pub struct DdrRam {
    regs: &'static mut regs::RegisterBlock,
    phy: phy::DdrPhy,
}

impl DdrRam {
    pub fn ddr_ram() -> Self {
        let self_ = DdrRam {
            regs: regs::RegisterBlock::ddrc(),
            phy: phy::DdrPhy::ddr_phy(),
        };
        self_
    }

    pub fn configure(&mut self, config: spd::GeneralConfig) {
        // assert DDRC reset
        crf_apb::RegisterBlock::unlocked(|crf_apb| {
            crf_apb.rst_ddr_ss.modify(|_, w| w.ddr_reset(true))
        });

        self.write_regs(config);

        // bring controller out of reset
        crf_apb::RegisterBlock::unlocked(|crf_apb| {
            crf_apb.rst_ddr_ss.modify(|_, w| w.ddr_reset(false))
        });
    }

    pub fn write_regs(&mut self, config: spd::GeneralConfig) {
        // master
        let device_config = match config.device_width {
            8 => regs::DeviceConfig::X8,
            16 => regs::DeviceConfig::X16,
            32 => regs::DeviceConfig::X32,
            _ => panic!("Invalid device width"),
        };
        let burst_rdwr = match config.burst_len() {
            4 => regs::BurstRdwr::Len4,
            8 => regs::BurstRdwr::Len8,
            16 => regs::BurstRdwr::Len16,
            _ => panic!("Invalid burst length"),
        };
        let bus_width = match config.bus_width {
            16 => regs::DataBusWidth::Quarter,
            32 => regs::DataBusWidth::Half,
            64 => regs::DataBusWidth::Full,
            _ => panic!("Invalid bus width"),
        };
        self.regs.master.write(
            regs::Master::zeroed()
                .device_config(device_config)
                .active_ranks(config.logical_ranks())
                .burst_rdwr(burst_rdwr)
                .data_bus_width(bus_width)
                .lpddr4(matches!(config.device_type, spd::DeviceType::LpDdr4))
                .ddr4(matches!(config.device_type, spd::DeviceType::Ddr4))
                .lpddr3(matches!(config.device_type, spd::DeviceType::LpDdr3))
                .ddr3(matches!(config.device_type, spd::DeviceType::Ddr3)),
        );

        // derate_en
        let rc_derate_value = ceilf(3.75 / config.ctl_clock_period_ns()) as u8;
        self.regs
            .derate_en
            .write(regs::DerateEn::zeroed().rc_derate_value(rc_derate_value));

        // power_ctrl
        self.regs.power_ctrl.write(
            regs::PowerCtrl::zeroed()
                .en_dfi_dram_clk_disable(config.clock_stop_en)
                .powerdown_en(config.power_down_en)
                .self_ref_en(config.self_ref_en),
        );

        // TODO: power_timing if enabled
        // refresh_ctrl1
        // extra division by 2 since that's what the registers want
        let t_refi_x32 = (config.ps_to_nck(config.t_refi()) / 64) as u16;
        // if a second rank is present, stagger the refreshes
        let timer1_start_value_x32 = if config.rank_addr_bits() == 1 {
            t_refi_x32 / 2
        } else {
            0
        };
        self.regs
            .refresh_ctrl1
            .write(regs::RefreshCtrl1::zeroed().timer1_start_value_x32(timer1_start_value_x32));

        // refresh_ctrl3
        let refresh_mode = match config.fine_granularity_ref_mode {
            spd::FineGranularityRefMode::X1 => 0b000,
            spd::FineGranularityRefMode::X2 => 0b001,
            spd::FineGranularityRefMode::X4 => 0b010,
        };
        self.regs
            .refresh_ctrl3
            .write(regs::RefreshCtrl3::zeroed().refresh_mode(refresh_mode));

        // refresh_timing
        let t_rfc_min = config.ps_to_nck(config.t_rfc_min_ps()).div_ceil(2) as u16;
        let mut t_rfc_nom_x32 = if config.temp_ref_range {
            t_refi_x32 / 2
        } else {
            t_refi_x32
        };
        t_rfc_nom_x32 = match config.fine_granularity_ref_mode {
            spd::FineGranularityRefMode::X1 => t_rfc_nom_x32.min(0xffe),
            spd::FineGranularityRefMode::X2 => (t_rfc_nom_x32 / 2).min(0x7ff),
            spd::FineGranularityRefMode::X4 => (t_rfc_nom_x32 / 4).min(0x3ff),
        };
        self.regs.refresh_timing.write(
            regs::RefreshTiming::zeroed()
                .t_rfc_min(t_rfc_min)
                .t_rfc_nom_x32(t_rfc_nom_x32),
        );

        // TODO: other ecc_cfg, crc_par_ctrl if enabled
        self.regs
            .ecc_cfg0
            .write(regs::EccCfg0::zeroed().ecc_mode((config.ecc_en as u8) << 2));
        self.regs.crc_par_ctrl1.write(
            regs::CrcParCtrl1::zeroed()
                .crc_enable(config.crc_en)
                .parity_enable(config.parity_en),
        );
        self.regs.crc_par_ctrl2.write(
            regs::CrcParCtrl2::zeroed()
                .t_par_alert_pw_max((config.speed_bin_mhz() * 3).div_ceil(100) as u16)
                .t_crc_alert_pw_max(5),
        );

        // init regs
        let post_cke_x1024 = (config.ps_to_nck(400_000) / 1024).div_ceil(2) as u16;
        let pre_cke_x1024 = (config.ps_to_nck(500_000_000) / 1024).div_ceil(2) as u16;
        self.regs.init0.write(
            regs::Init0::zeroed()
                .post_cke_x1024(post_cke_x1024)
                .pre_cke_x1024(pre_cke_x1024),
        );
        let dram_rstn_x1024 = (config.ps_to_nck(100_000) / 1024).div_ceil(2) as u16;
        self.regs
            .init1
            .write(regs::Init1::zeroed().dram_rstn_x1024(dram_rstn_x1024));
        self.regs
            .init3
            .write(regs::Init3::zeroed().mr(config.mr0()));

        // dimm_ctrl
        let dimm_addr_mirr_en = match config.module_config {
            spd::ModuleConfig::Unbuffered(mod_config) => mod_config.rank_1_mirrored,
            _ => false,
        };
        self.regs
            .dimm_ctrl
            .write(regs::DimmCtrl::zeroed().dimm_addr_mirr_en(dimm_addr_mirr_en));

        // dram_tmg0
        let t_faw = config.ps_to_nck(config.t_faw_min_ps).div_ceil(2) as u8;
        self.regs
            .dram_tmg0
            .write(regs::DramTmg0::zeroed().t_faw(t_faw));

        // dram_tmg1
        let t_xp = (config.t_xp_nck() + config.parity_latency_nck()).div_ceil(2) as u8;
        let t_rc = config.ps_to_nck(config.t_rc_min_ps).div_ceil(2) as u8;
        self.regs
            .dram_tmg1
            .write(regs::DramTmg1::zeroed().t_xp(t_xp).t_rc(t_rc));

        // dram_tmg2
        let write_latency = config.write_latency_nck().div_ceil(2) as u8;
        let read_latency = config.read_latency_nck().div_ceil(2) as u8;
        self.regs.dram_tmg2.write(
            regs::DramTmg2::zeroed()
                .write_latency(write_latency)
                .read_latency(read_latency),
        );

        // dram_tmg4
        let t_rcd = (config.ps_to_nck(config.t_rcd_min_ps) - config.additive_latency_nck())
            .div_ceil(2) as u8;
        let t_ccd = config.ps_to_nck(config.t_ccd_l_min_ps).div_ceil(2) as u8;
        let t_rrd = config.ps_to_nck(config.t_rrd_l_min_ps).div_ceil(2) as u8;
        let t_rp = (config.t_rp_min_ps.div_ceil(config.t_ckavg_min_ps) / 2 + 1) as u8;
        self.regs.dram_tmg4.write(
            regs::DramTmg4::zeroed()
                .t_rcd(t_rcd)
                .t_ccd(t_ccd)
                .t_rrd(t_rrd)
                .t_rp(t_rp),
        );

        // dram_tmg8
        let t_xs_fast_x32 =
            (config.ps_to_nck(config.t_xs_fast_min_ns() * 1000) / 32).div_ceil(2) as u8;
        let t_xs_abort_x32 =
            (config.ps_to_nck(config.t_xs_abort_min_ns() * 1000) / 32).div_ceil(2) as u8;
        let t_xs_dll_x32 = (config.t_xs_dll_min_nck() / 32).div_ceil(2) as u8;
        let t_xs_x32 = (config.ps_to_nck(config.t_xs_min_ns() * 1000) / 32).div_ceil(2) as u8;
        self.regs.dram_tmg8.write(
            regs::DramTmg8::zeroed()
                .t_xs_fast_x32(t_xs_fast_x32)
                .t_xs_abort_x32(t_xs_abort_x32)
                .t_xs_dll_x32(t_xs_dll_x32)
                .t_xs_x32(t_xs_x32),
        );

        // TODO: dram_tmg9 (wr preamble = 0?)

        // dram_tmg12
        let t_mrd_pda = config.t_mrd_pda_min_nck().div_ceil(2) as u8;
        self.regs
            .dram_tmg12
            .write(regs::DramTmg12::zeroed().t_mrd_pda(t_mrd_pda));

        // address map
        self.regs
            .addr_map0
            .write(regs::AddrMap0::zeroed().addrmap_cs_bit0(0x1f));
        let bank_addr_map = config.bank_addr_map();
        self.regs.addr_map1.write(
            regs::AddrMap1::zeroed()
                .addrmap_bank_b2(bank_addr_map[2] as u8)
                .addrmap_bank_b1(bank_addr_map[1] as u8)
                .addrmap_bank_b0(bank_addr_map[0] as u8),
        );
        let col_addr_map = config.col_addr_map();
        self.regs.addr_map2.write(
            regs::AddrMap2::zeroed()
                .addrmap_col_b5(col_addr_map[5] as u8)
                .addrmap_col_b4(col_addr_map[4] as u8)
                .addrmap_col_b3(col_addr_map[3] as u8)
                .addrmap_col_b2(col_addr_map[2] as u8),
        );
        self.regs.addr_map3.write(
            regs::AddrMap3::zeroed()
                .addrmap_col_b9(col_addr_map[9] as u8)
                .addrmap_col_b8(col_addr_map[8] as u8)
                .addrmap_col_b7(col_addr_map[7] as u8)
                .addrmap_col_b6(col_addr_map[6] as u8),
        );
        self.regs.addr_map4.write(
            regs::AddrMap4::zeroed()
                .addrmap_col_b11(col_addr_map[11] as u8)
                .addrmap_col_b10(col_addr_map[10] as u8),
        );
        let row_addr_map = config.row_addr_map();
        self.regs.addr_map5.write(
            regs::AddrMap5::zeroed()
                .addrmap_row_b11(row_addr_map[11] as u8)
                .addrmap_row_b2_10(0xf)
                .addrmap_row_b1(row_addr_map[1] as u8)
                .addrmap_row_b0(row_addr_map[0] as u8),
        );
        self.regs.addr_map6.write(
            regs::AddrMap6::zeroed()
                .addrmap_row_b15(row_addr_map[15] as u8)
                .addrmap_row_b14(row_addr_map[14] as u8)
                .addrmap_row_b13(row_addr_map[13] as u8)
                .addrmap_row_b12(row_addr_map[12] as u8),
        );
        self.regs.addr_map7.write(
            regs::AddrMap7::zeroed()
                .addrmap_row_b17(row_addr_map[17] as u8)
                .addrmap_row_b16(row_addr_map[16] as u8),
        );
        let bg_addr_map = config.bg_addr_map();
        self.regs.addr_map8.write(
            regs::AddrMap8::zeroed()
                .addrmap_bg_b1(bg_addr_map[1] as u8)
                .addrmap_bg_b0(bg_addr_map[0] as u8),
        );
        self.regs.addr_map9.write(
            regs::AddrMap9::zeroed()
                .addrmap_row_b5(row_addr_map[5] as u8)
                .addrmap_row_b4(row_addr_map[4] as u8)
                .addrmap_row_b3(row_addr_map[3] as u8)
                .addrmap_row_b2(row_addr_map[2] as u8),
        );
        self.regs.addr_map10.write(
            regs::AddrMap10::zeroed()
                .addrmap_row_b9(row_addr_map[9] as u8)
                .addrmap_row_b8(row_addr_map[8] as u8)
                .addrmap_row_b7(row_addr_map[7] as u8)
                .addrmap_row_b6(row_addr_map[6] as u8),
        );
        self.regs
            .addr_map11
            .write(regs::AddrMap11::zeroed().addrmap_row_b10(row_addr_map[10] as u8));

        // dq map
        self.regs.dq_map0.write(
            regs::DqMap0::zeroed()
                .dq_nibble_map_12_15(config.dq_map[3])
                .dq_nibble_map_8_11(config.dq_map[2])
                .dq_nibble_map_4_7(config.dq_map[1])
                .dq_nibble_map_0_3(config.dq_map[0]),
        );
        self.regs.dq_map1.write(
            regs::DqMap1::zeroed()
                .dq_nibble_map_28_31(config.dq_map[7])
                .dq_nibble_map_24_27(config.dq_map[6])
                .dq_nibble_map_20_23(config.dq_map[5])
                .dq_nibble_map_16_19(config.dq_map[4]),
        );
        self.regs.dq_map2.write(
            regs::DqMap2::zeroed()
                .dq_nibble_map_44_47(config.dq_map[13])
                .dq_nibble_map_40_43(config.dq_map[12])
                .dq_nibble_map_36_39(config.dq_map[11])
                .dq_nibble_map_32_35(config.dq_map[10]),
        );
        self.regs.dq_map3.write(
            regs::DqMap3::zeroed()
                .dq_nibble_map_60_63(config.dq_map[17])
                .dq_nibble_map_56_59(config.dq_map[16])
                .dq_nibble_map_52_55(config.dq_map[15])
                .dq_nibble_map_48_51(config.dq_map[14]),
        );
        self.regs.dq_map4.write(
            regs::DqMap4::zeroed()
                .dq_nibble_map_cb_4_7(config.dq_map[9])
                .dq_nibble_map_cb_0_3(config.dq_map[8]),
        );
    }

    pub fn ptr<T>(&mut self) -> *mut T {
        // TODO: starts at zero but runtime checks for null pointers don't like that
        0x0010_0000 as *mut _
    }

    pub fn size(&self) -> usize {
        // TODO: DDR_LO is 2 GB, but there's another 2 GB (for ZCU 111) at the high addr
        2047 * 1024 * 1024
    }

    pub fn memtest(&mut self) {
        let slice = unsafe { core::slice::from_raw_parts_mut(self.ptr(), self.size()) };
        let patterns: &'static [u32] = &[0xffff_ffff, 0x5555_5555, 0xaaaa_aaaa, 0];
        for (i, pattern) in patterns.iter().enumerate() {
            println!("memtest phase {}: {:#08X}", i, pattern);

            println!("writing...");
            for megabyte in 0..slice.len() / (1024 * 1024) {
                let start = megabyte * 1024 * 1024 / 4;
                let end = (megabyte + 1) * 1024 * 1024 / 4;
                for b in slice[start..end].iter_mut() {
                    *b = *pattern;
                }

                print!("\r{} MB", megabyte);
            }
            println!(" Ok");

            println!("reading...");
            let expected = *pattern;
            for megabyte in 0..slice.len() / (1024 * 1024) {
                let start = megabyte * 1024 * 1024 / 4;
                let end = (megabyte + 1) * 1024 * 1024 / 4;
                for b in slice[start..end].iter_mut() {
                    let read: u32 = *b;
                    if read != expected {
                        println!(
                            "{:08X}: expected {:08X}, read {:08X}",
                            b as *mut _ as usize, expected, read
                        );
                    };
                }

                print!("\r{} MB", megabyte);
            }
            println!(" Ok");
        }
    }
}
