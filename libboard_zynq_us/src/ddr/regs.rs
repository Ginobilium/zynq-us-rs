use libregister::{register, register_at, register_bit, register_bits, register_bits_typed};
use volatile_register::{RO, RW};

#[allow(unused)]
#[repr(u8)]
pub enum DeviceConfig {
    X4 = 0b00,
    X8 = 0b01,
    X16 = 0b10,
    X32 = 0b11,
}

#[allow(unused)]
#[repr(u8)]
pub enum BurstRdwr {
    Len4 = 0b0010,
    Len8 = 0b0100,
    Len16 = 0b1000,
}

#[allow(unused)]
#[repr(u8)]
pub enum DataBusWidth {
    Full = 0b00,
    Half = 0b01,
    Quarter = 0b10,
}

#[repr(C)]
pub struct RegisterBlock {
    pub master: Master,
    pub status: Status,
    unused1: [u32; 2],
    pub mode_ctrl0: ModeCtrl0,
    pub mode_ctrl1: RW<u32>,
    pub mode_status: RO<u32>,
    pub mode_ctrl2: RW<u32>,
    pub derate_en: DerateEn,
    pub derate_interval: DerateInterval,
    unused2: [u32; 2],
    pub power_ctrl: PowerCtrl,
    pub power_timing: PowerTiming,
    pub hw_lp_ctrl: RW<u32>,
    unused3: [u32; 5],
    pub refresh_ctrl0: RefreshCtrl0,
    pub refresh_ctrl1: RefreshCtrl1,
    unused4: [u32; 1],
    pub refresh_ctrl3: RefreshCtrl3,
    pub refresh_timing: RefreshTiming,
    unused5: [u32; 2],
    pub ecc_cfg0: RW<u32>,
    pub ecc_cfg1: RW<u32>,
    pub ecc_status: RO<u32>,
    pub ecc_clr: RW<u32>,
    pub ecc_err_cnt: RO<u32>,
    pub ecc_caddr0: RO<u32>,
    pub ecc_caddr1: RO<u32>,
    pub ecc_csynd0: RO<u32>,
    pub ecc_csynd1: RO<u32>,
    pub ecc_csynd2: RO<u32>,
    pub ecc_cmask0: RO<u32>,
    pub ecc_cmask1: RO<u32>,
    pub ecc_cmask2: RO<u32>,
    pub ecc_uaddr0: RO<u32>,
    pub ecc_uaddr1: RO<u32>,
    pub ecc_usynd0: RO<u32>,
    pub ecc_usynd1: RO<u32>,
    pub ecc_usynd2: RO<u32>,
    pub ecc_poison_addr0: RW<u32>,
    pub ecc_poison_addr1: RW<u32>,
    pub crc_par_ctrl0: RW<u32>,
    pub crc_par_ctrl1: RW<u32>,
    pub crc_par_ctrl2: RW<u32>,
    pub crc_par_status: RO<u32>,
    pub init0: RW<u32>,
    pub init1: RW<u32>,
    pub init2: RW<u32>,
    pub init3: RW<u32>,
    pub init4: RW<u32>,
    pub init5: RW<u32>,
    pub init6: RW<u32>,
    pub init7: RW<u32>,
    pub dimm_ctrl: RW<u32>,
    pub rank_ctrl: RW<u32>,
    unused6: [u32; 2],
    pub dram_tmg0: RW<u32>,
    pub dram_tmg1: RW<u32>,
    pub dram_tmg2: RW<u32>,
    pub dram_tmg3: RW<u32>,
    pub dram_tmg4: RW<u32>,
    pub dram_tmg5: RW<u32>,
    pub dram_tmg6: RW<u32>,
    pub dram_tmg7: RW<u32>,
    pub dram_tmg8: RW<u32>,
    pub dram_tmg9: RW<u32>,
    pub dram_tmg10: RW<u32>,
    pub dram_tmg11: RW<u32>,
    pub dram_tmg12: RW<u32>,
    pub dram_tmg13: RW<u32>,
    pub dram_tmg14: RW<u32>,
    unused7: [u32; 17],
    pub zq_ctrl0: RW<u32>,
    pub zq_ctrl1: RW<u32>,
    pub zq_ctrl2: RW<u32>,
    pub zq_status: RO<u32>,
    pub dfi_tmg0: RW<u32>,
    pub dfi_tmg1: RW<u32>,
    pub dfi_lp_cfg0: RW<u32>,
    pub dfi_lp_cfg1: RW<u32>,
    pub dfi_update0: RW<u32>,
    pub dfi_update1: RW<u32>,
    pub dfi_update2: RW<u32>,
    unused8: [u32; 1],
    pub dfi_misc: RW<u32>,
    pub dfi_tmg2: RW<u32>,
    unused9: [u32; 2],
    pub dbi_ctrl: RW<u32>,
    unused10: [u32; 15],
    pub addr_map0: RW<u32>,
    pub addr_map1: RW<u32>,
    pub addr_map2: RW<u32>,
    pub addr_map3: RW<u32>,
    pub addr_map4: RW<u32>,
    pub addr_map5: RW<u32>,
    pub addr_map6: RW<u32>,
    pub addr_map7: RW<u32>,
    pub addr_map8: RW<u32>,
    pub addr_map9: RW<u32>,
    pub addr_map10: RW<u32>,
    pub addr_map11: RW<u32>,
    unused11: [u32; 4],
    pub odt_cfg: RW<u32>,
    pub odt_map: RW<u32>,
    unused12: [u32; 2],
    pub sched: RW<u32>,
    pub sched1: RW<u32>,
    unused13: [u32; 1],
    pub perf_hpr1: RW<u32>,
    unused14: [u32; 1],
    pub perf_lpr1: RW<u32>,
    unused15: [u32; 1],
    pub perf_wr1: RW<u32>,
    unused16: [u32; 1],
    pub perf_vpr1: RW<u32>,
    pub perf_vpw1: RW<u32>,
    unused17: [u32; 1],
    pub dq_map0: RW<u32>,
    pub dq_map1: RW<u32>,
    pub dq_map2: RW<u32>,
    pub dq_map3: RW<u32>,
    pub dq_map4: RW<u32>,
    pub dq_map5: RW<u32>,
    unused18: [u32; 3],
    pub dbg1: RW<u32>,
    pub dbg_cam: RO<u32>,
    pub dbg_cmd: RW<u32>,
    pub dbg_status: RO<u32>,
    unused19: [u32; 3],
    pub sw_ctrl: RW<u32>,
    pub sw_status: RO<u32>,
    unused20: [u32; 17],
    pub poison_cfg: RW<u32>,
    pub poison_status: RO<u32>,
    unused21: [u32; 34],
    pub port_status: RO<u32>,
    pub port_common_cfg: RW<u32>,
    pub port0_cfgr: RW<u32>,
    pub port0_cfgw: RW<u32>,
    unused22: [u32; 33],
    pub port0_ctrl: RW<u32>,
    pub port0_rqos_cfg0: RW<u32>,
    pub port0_rqos_cfg1: RW<u32>,
    pub port0_wqos_cfg0: RW<u32>,
    pub port0_wqos_cfg1: RW<u32>,
    unused23: [u32; 4],
    pub port1_cfgr: RW<u32>,
    pub port1_cfgw: RW<u32>,
    unused24: [u32; 33],
    pub port1_ctrl: RW<u32>,
    pub port1_rqos_cfg0: RW<u32>,
    pub port1_rqos_cfg1: RW<u32>,
    pub port1_wqos_cfg0: RW<u32>,
    pub port1_wqos_cfg1: RW<u32>,
    unused25: [u32; 4],
    pub port2_cfgr: RW<u32>,
    pub port2_cfgw: RW<u32>,
    unused26: [u32; 33],
    pub port2_ctrl: RW<u32>,
    pub port2_rqos_cfg0: RW<u32>,
    pub port2_rqos_cfg1: RW<u32>,
    pub port2_wqos_cfg0: RW<u32>,
    pub port2_wqos_cfg1: RW<u32>,
    unused27: [u32; 4],
    pub port3_cfgr: RW<u32>,
    pub port3_cfgw: RW<u32>,
    unused28: [u32; 33],
    pub port3_ctrl: RW<u32>,
    pub port3_rqos_cfg0: RW<u32>,
    pub port3_rqos_cfg1: RW<u32>,
    pub port3_wqos_cfg0: RW<u32>,
    pub port3_wqos_cfg1: RW<u32>,
    unused29: [u32; 4],
    pub port4_cfgr: RW<u32>,
    pub port4_cfgw: RW<u32>,
    unused30: [u32; 33],
    pub port4_ctrl: RW<u32>,
    pub port4_rqos_cfg0: RW<u32>,
    pub port4_rqos_cfg1: RW<u32>,
    pub port4_wqos_cfg0: RW<u32>,
    pub port4_wqos_cfg1: RW<u32>,
    unused31: [u32; 4],
    pub port5_cfgr: RW<u32>,
    pub port5_cfgw: RW<u32>,
    unused32: [u32; 33],
    pub port5_ctrl: RW<u32>,
    pub port5_rqos_cfg0: RW<u32>,
    pub port5_rqos_cfg1: RW<u32>,
    pub port5_wqos_cfg0: RW<u32>,
    pub port5_wqos_cfg1: RW<u32>,
    unused33: [u32; 444],
    pub sar_base0: RW<u32>,
    pub sar_size0: RW<u32>,
    pub sar_base1: RW<u32>,
    pub sar_size1: RW<u32>,
    unused34: [u32; 1092],
    pub derate_int_shadow: RW<u32>,
    unused35: [u32; 2],
    pub refresh_ctrl0_shadow: RW<u32>,
    unused36: [u32; 4],
    pub refresh_timing_shadow: RW<u32>,
    unused37: [u32; 29],
    pub init3_shadow: RW<u32>,
    pub init4_shadow: RW<u32>,
    unused38: [u32; 1],
    pub init6_shadow: RW<u32>,
    pub init7_shadow: RW<u32>,
    unused39: [u32; 4],
    pub dram_tmg0_shadow: RW<u32>,
    pub dram_tmg1_shadow: RW<u32>,
    pub dram_tmg2_shadow: RW<u32>,
    pub dram_tmg3_shadow: RW<u32>,
    pub dram_tmg4_shadow: RW<u32>,
    pub dram_tmg5_shadow: RW<u32>,
    pub dram_tmg6_shadow: RW<u32>,
    pub dram_tmg7_shadow: RW<u32>,
    pub dram_tmg8_shadow: RW<u32>,
    pub dram_tmg9_shadow: RW<u32>,
    pub dram_tmg10_shadow: RW<u32>,
    pub dram_tmg11_shadow: RW<u32>,
    pub dram_tmg12_shadow: RW<u32>,
    pub dram_tmg13_shadow: RW<u32>,
    pub dram_tmg14_shadow: RW<u32>,
    unused40: [u32; 17],
    pub zq_ctrl0_shadow: RW<u32>,
    unused41: [u32; 3],
    pub dfi_tmg0_shadow: RW<u32>,
    pub dfi_tmg1_shadow: RW<u32>,
    unused42: [u32; 7],
    pub dfi_tmg2_shadow: RW<u32>,
    unused43: [u32; 34],
    pub odt_cfg_shadow: RW<u32>,
}
register_at!(RegisterBlock, 0xFD07_0000, ddrc);

register!(master, Master, RW, u32);
register_bits_typed!(master, device_config, u8, DeviceConfig, 30, 31);
register_bit!(master, freq_mode, 29);
register_bits!(master, active_ranks, u8, 24, 25);
register_bits_typed!(master, burst_rdwr, u8, BurstRdwr, 16, 19);
register_bit!(master, dll_off_mode, 15);
register_bits_typed!(master, data_bus_width, u8, DataBusWidth, 12, 13);
register_bit!(master, en_2t_timing_mode, 10);
register_bit!(master, lpddr4, 5);
register_bit!(master, ddr4, 4);
register_bit!(master, lpddr3, 3);
register_bit!(master, ddr3, 0);

register!(status, Status, RO, u32);
register_bits!(status, selfref_state, u8, 8, 9);
register_bits!(status, selfref_type, u8, 4, 5);
register_bits!(status, operating_mode, u8, 0, 2);

register!(mode_ctrl0, ModeCtrl0, RW, u32);
register_bit!(mode_ctrl0, mr_wr, 31);
register_bits!(mode_ctrl0, mr_addr, u8, 12, 15);
register_bits!(mode_ctrl0, mr_rank, u8, 4, 5);
register_bit!(mode_ctrl0, sw_init_int, 3);
register_bit!(mode_ctrl0, pda_en, 2);
register_bit!(mode_ctrl0, mpr_en, 1);
register_bit!(mode_ctrl0, mr_type, 0);

register!(derate_en, DerateEn, RW, u32);
register_bits!(derate_en, rc_derate_value, u8, 8, 9);
register_bits!(derate_en, derate_byte, u8, 4, 7);
register_bit!(derate_en, derate_value, 1);
register_bit!(derate_en, derate_enable, 0);

register!(derate_interval, DerateInterval, RW, u32);
register_bits!(derate_interval, mr4_read_interval, u32, 0, 31);

register!(power_ctrl, PowerCtrl, RW, u32);
register_bit!(power_ctrl, stay_in_self_ref, 6);
register_bit!(power_ctrl, sel_fref_sw, 5);
register_bit!(power_ctrl, mpsm_en, 4);
register_bit!(power_ctrl, en_dfi_dram_clk_disable, 3);
register_bit!(power_ctrl, deep_powerdown_en, 2);
register_bit!(power_ctrl, powerdown_en, 1);
register_bit!(power_ctrl, self_ref_en, 0);

register!(power_timing, PowerTiming, RW, u32);
register_bits!(power_timing, self_ref_to_x32, u8, 16, 23);
register_bits!(power_timing, t_dpd_x4096, u8, 8, 15);
register_bits!(power_timing, powerdown_to_x32, u8, 0, 4);

register!(refresh_ctrl0, RefreshCtrl0, RW, u32);
register_bits!(refresh_ctrl0, refresh_margin, u8, 20, 23);
register_bits!(refresh_ctrl0, refresh_to_x32, u8, 12, 16);
register_bits!(refresh_ctrl0, refresh_burst, u8, 4, 8);
register_bit!(refresh_ctrl0, per_bank_refresh, 2);

register!(refresh_ctrl1, RefreshCtrl1, RW, u32);
register_bits!(refresh_ctrl1, timer1_start_value_x32, u16, 16, 27);
register_bits!(refresh_ctrl1, timer0_start_value_x32, u16, 0, 11);

register!(refresh_ctrl3, RefreshCtrl3, RW, u32);
register_bits!(refresh_ctrl3, refresh_mode, u8, 4, 6);
register_bit!(refresh_ctrl3, refresh_update_level, 1);
register_bit!(refresh_ctrl3, disable_auto_refresh, 0);

register!(refresh_timing, RefreshTiming, RW, u32);
register_bits!(refresh_timing, t_rfc_nom_x32, u16, 16, 27);
register_bit!(refresh_timing, lpddr3_trefbw_en, 15);
register_bits!(refresh_timing, t_rfc_min, u16, 0, 9);