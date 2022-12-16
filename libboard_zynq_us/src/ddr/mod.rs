use libregister::{RegisterRW, RegisterW, RegisterR};
use super::slcr::{common::Unlocked, crf_apb};
mod phy;
mod regs;

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
            phy: phy::DdrPhy::ddr_phy()
        };
        self_
     }

    pub fn init(&mut self) {
        self.configure();
        self.phy.configure();
        self.phy.init_plls();
        self.phy.execute_zcal_dcal();
        unsafe {
            // enable dfi_init_complete signal
            self.regs.dfi_misc.write(0x00000001);
            // enable quasi-dynamic register programming
            self.regs.sw_ctrl.write(0x00000001);
        }
        let mut timeout = 1_000_000;
        let mut status_read = self.regs.status.read();
        // wait for operating mode to read "Normal"
        while timeout > 0 && status_read.operating_mode() != 0b001 {
            status_read = self.regs.status.read();
            timeout -= 1;
        }
        
        self.phy.execute_training();

        unsafe { self.regs.zq_ctrl0.write(0x01000040); }
        self.regs.refresh_ctrl3.write(regs::RefreshCtrl3::zeroed());
        unsafe { self.phy.regs.pgcr1.modify(|cur| cur & !(1 << 6)); }
    }
    pub fn configure(&mut self) {
        // assert ddr reset
        crf_apb::RegisterBlock::unlocked(|slcr| 
            slcr.rst_ddr_ss.modify(|_, w| w.ddr_reset(true))
        );
        self.regs.master.write(
            regs::Master::zeroed()
                .device_config(regs::DeviceConfig::X16)
                .active_ranks(1)
                .burst_rdwr(regs::BurstRdwr::Len8)
                .data_bus_width(regs::DataBusWidth::Full)
                .ddr4(true)
        );
        self.regs.mode_ctrl0.write(
            regs::ModeCtrl0::zeroed().mr_rank(3) // not sure why this isn't just 1
        );
        self.regs.derate_en.write(regs::DerateEn::zeroed().rc_derate_value(2));
        self.regs.derate_interval.write(
            regs::DerateInterval::zeroed().mr4_read_interval(0x00800000)
        );
        self.regs.power_ctrl.write(regs::PowerCtrl::zeroed());
        self.regs.power_timing.write(
            regs::PowerTiming::zeroed()
                .self_ref_to_x32(0x40)
                .t_dpd_x4096(0x84)
                .powerdown_to_x32(0x10)
        );
        self.regs.refresh_ctrl0.write(
            regs::RefreshCtrl0::zeroed()
                .refresh_margin(0x2)
                .refresh_to_x32(0x10)
                .refresh_burst(0)
                .per_bank_refresh(false)
        );
        self.regs.refresh_ctrl1.write(regs::RefreshCtrl1::zeroed());
        self.regs.refresh_ctrl3.write(regs::RefreshCtrl3::zeroed().disable_auto_refresh(true));
        // target of ~7.8 us (64 ms / 8192) in units of 32 clock cycles
        self.regs.refresh_timing.write(
            regs::RefreshTiming::zeroed()
                .t_rfc_nom_x32(0x81)
                .lpddr3_trefbw_en(true)
                .t_rfc_min(0xbb)
        );
        unsafe {
            self.regs.ecc_cfg0.write(0x00000010);
            self.regs.ecc_cfg1.write(0x00000000);

            self.regs.crc_par_ctrl1.write(0x10000200);
            self.regs.crc_par_ctrl2.write(0x0040051F);
            
            self.regs.init0.write(0x00020106);
            self.regs.init1.write(0x00020000);
            self.regs.init2.write(0x00002305);
            self.regs.init3.write(0x07300301);
            self.regs.init4.write(0x00200200);
            self.regs.init5.write(0x00210004);
            self.regs.init6.write(0x000006C0);
            self.regs.init7.write(0x08190000);
            
            self.regs.dimm_ctrl.write(0x00000010);
            self.regs.rank_ctrl.write(0x0000066F);
            
            self.regs.dram_tmg0.write(0x11102412);
            self.regs.dram_tmg1.write(0x0004041A);
            self.regs.dram_tmg2.write(0x0708060D);
            self.regs.dram_tmg3.write(0x0050400C);
            self.regs.dram_tmg4.write(0x08030409);
            self.regs.dram_tmg5.write(0x06060403);
            self.regs.dram_tmg6.write(0x01010004);
            self.regs.dram_tmg7.write(0x00000606);
            self.regs.dram_tmg8.write(0x04040D07);
            self.regs.dram_tmg9.write(0x0002030B);
            self.regs.dram_tmg11.write(0x1207010E);
            self.regs.dram_tmg12.write(0x00020608);
            
            self.regs.zq_ctrl0.write(0x81000040);
            self.regs.zq_ctrl1.write(0x020196E5);
            
            self.regs.dfi_tmg0.write(0x048B820B);
            self.regs.dfi_tmg1.write(0x00030304);
            self.regs.dfi_lp_cfg0.write(0x07000101);
            self.regs.dfi_lp_cfg1.write(0x00000021);
            self.regs.dfi_update0.write(0x00400003);
            self.regs.dfi_update1.write(0x00C800FF);
            self.regs.dfi_misc.write(0x00000000);
            self.regs.dfi_tmg2.write(0x00000909);
            self.regs.dbi_ctrl.write(0x00000001);
            
            self.regs.addr_map0.write(0x0000001F);
            self.regs.addr_map1.write(0x001F0909);
            self.regs.addr_map2.write(0x01010100);
            self.regs.addr_map3.write(0x01010101);
            self.regs.addr_map4.write(0x00000F0F);
            self.regs.addr_map5.write(0x070F0707);
            self.regs.addr_map6.write(0x07070707);
            self.regs.addr_map7.write(0x00000F0F);
            self.regs.addr_map8.write(0x00001F01);
            self.regs.addr_map9.write(0x07070707);
            self.regs.addr_map10.write(0x07070707);
            self.regs.addr_map11.write(0x00000007);
            
            self.regs.odt_cfg.write(0x06000600);
            self.regs.odt_map.write(0x00000001);
            self.regs.sched.write(0x01002001);
            self.regs.perf_lpr1.write(0x08000040);
            self.regs.perf_wr1.write(0x08000040);
            
            self.regs.dq_map0.write(0x00000000);
            self.regs.dq_map1.write(0x00000000);
            self.regs.dq_map2.write(0x00000000);
            self.regs.dq_map3.write(0x00000000);
            self.regs.dq_map4.write(0x00000000);
            self.regs.dq_map5.write(0x00000001);
            // dbg0 not documented
            self.regs.dbg_cmd.write(0x00000000);
            self.regs.sw_ctrl.write(0x00000000);
            self.regs.port_common_cfg.write(0x00000001);
            
            self.regs.port0_cfgr.write(0x0000200F);
            self.regs.port0_cfgw.write(0x0000200F);
            self.regs.port0_ctrl.write(0x00000001);
            self.regs.port0_rqos_cfg0.write(0x0020000B);
            self.regs.port0_rqos_cfg1.write(0x00000000);
            
            self.regs.port1_cfgr.write(0x0000200F);
            self.regs.port1_cfgw.write(0x0000200F);
            self.regs.port1_ctrl.write(0x00000001);
            self.regs.port1_rqos_cfg0.write(0x02000B03);
            self.regs.port1_rqos_cfg1.write(0x00000000);

            self.regs.port2_cfgr.write(0x0000200F);
            self.regs.port2_cfgw.write(0x0000200F);
            self.regs.port2_ctrl.write(0x00000001);
            self.regs.port2_rqos_cfg0.write(0x02000B03);
            self.regs.port2_rqos_cfg1.write(0x00000000);

            self.regs.port3_cfgr.write(0x0000200F);
            self.regs.port3_cfgw.write(0x0000200F);
            self.regs.port3_ctrl.write(0x00000001);
            self.regs.port3_rqos_cfg0.write(0x00100003);
            self.regs.port3_rqos_cfg1.write(0x0000004F);
            self.regs.port3_wqos_cfg0.write(0x00100003);
            self.regs.port3_wqos_cfg1.write(0x0000004F);

            self.regs.port4_cfgr.write(0x0000200F);
            self.regs.port4_cfgw.write(0x0000200F);
            self.regs.port4_ctrl.write(0x00000001);
            self.regs.port4_rqos_cfg0.write(0x00100003);
            self.regs.port4_rqos_cfg1.write(0x0000004F);
            self.regs.port4_wqos_cfg0.write(0x00100003);
            self.regs.port4_wqos_cfg1.write(0x0000004F);

            self.regs.port5_cfgr.write(0x0000200F);
            self.regs.port5_cfgw.write(0x0000200F);
            self.regs.port5_ctrl.write(0x00000001);
            self.regs.port5_rqos_cfg0.write(0x00100003);
            self.regs.port5_rqos_cfg1.write(0x0000004F);
            self.regs.port5_wqos_cfg0.write(0x00100003);
            self.regs.port5_wqos_cfg1.write(0x0000004F);
            
            self.regs.sar_base0.write(0x00000000);
            self.regs.sar_size0.write(0x00000000);
            self.regs.sar_base1.write(0x00000010);
            self.regs.sar_size1.write(0x0000000F);

            self.regs.dfi_tmg0_shadow.write(0x07828002);
        }
        // bring controller out of reset
        crf_apb::RegisterBlock::unlocked(|slcr| {
            slcr.rst_ddr_ss.write(crf_apb::RstDdrSS::zeroed())
        });
    }
}
