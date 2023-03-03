use libregister::{register, register_at, register_bit, register_bits, register_bits_typed};
use volatile_register::{RO, RW};

#[allow(unused)]
#[repr(u8)]
pub enum DeviceConfig {
    Reserved = 0b00,
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
    unused4: [u32; 2],
    pub refresh_ctrl3: RefreshCtrl3,
    pub refresh_timing: RefreshTiming,
    unused5: [u32; 2],
    pub ecc_cfg0: EccCfg0,
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
    pub crc_par_ctrl1: CrcParCtrl1,
    pub crc_par_ctrl2: CrcParCtrl2,
    pub crc_par_status: RO<u32>,
    pub init0: Init0,
    pub init1: Init1,
    pub init2: Init2,
    pub init3: Init3,
    pub init4: Init4,
    pub init5: Init5,
    pub init6: Init6,
    pub init7: Init7,
    pub dimm_ctrl: DimmCtrl,
    pub rank_ctrl: RW<u32>,
    unused6: [u32; 2],
    pub dram_tmg0: DramTmg0,
    pub dram_tmg1: DramTmg1,
    pub dram_tmg2: DramTmg2,
    pub dram_tmg3: DramTmg3,
    pub dram_tmg4: DramTmg4,
    pub dram_tmg5: RW<u32>,
    pub dram_tmg6: RW<u32>,
    pub dram_tmg7: RW<u32>,
    pub dram_tmg8: DramTmg8,
    pub dram_tmg9: DramTmg9,
    pub dram_tmg10: RW<u32>,
    pub dram_tmg11: RW<u32>,
    pub dram_tmg12: DramTmg12,
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
    pub addr_map0: AddrMap0,
    pub addr_map1: AddrMap1,
    pub addr_map2: AddrMap2,
    pub addr_map3: AddrMap3,
    pub addr_map4: AddrMap4,
    pub addr_map5: AddrMap5,
    pub addr_map6: AddrMap6,
    pub addr_map7: AddrMap7,
    pub addr_map8: AddrMap8,
    pub addr_map9: AddrMap9,
    pub addr_map10: AddrMap10,
    pub addr_map11: AddrMap11,
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
    pub dq_map0: DqMap0,
    pub dq_map1: DqMap1,
    pub dq_map2: DqMap2,
    pub dq_map3: DqMap3,
    pub dq_map4: DqMap4,
    pub dq_map5: DqMap5,
    unused18: [u32; 26],
    pub dbg0: RW<u32>,
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
    unused35: [u32; 10],
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
register_bit!(power_ctrl, self_ref_sw, 5);
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

register!(ecc_cfg0, EccCfg0, RW, u32);
register_bit!(ecc_cfg0, dis_scrub, 4);
register_bits!(ecc_cfg0, ecc_mode, u8, 0, 2);

register!(crc_par_ctrl1, CrcParCtrl1, RW, u32);
// TODO: other fields
register_bit!(crc_par_ctrl1, crc_enable, 4);
register_bit!(crc_par_ctrl1, parity_enable, 0);

register!(crc_par_ctrl2, CrcParCtrl2, RW, u32);
register_bits!(crc_par_ctrl2, t_par_alert_pw_max, u16, 16, 24);
register_bits!(crc_par_ctrl2, t_crc_alert_pw_max, u8, 8, 12);
register_bits!(crc_par_ctrl2, retry_fifo_max_hold_timer_x4, u8, 0, 5);

register!(init0, Init0, RW, u32);
register_bits!(init0, skip_dram_init, u8, 30, 31);
register_bits!(init0, post_cke_x1024, u16, 16, 25);
register_bits!(init0, pre_cke_x1024, u16, 0, 11);

register!(init1, Init1, RW, u32);
register_bits!(init1, dram_rstn_x1024, u16, 16, 24);
register_bits!(init1, final_wait_x32, u8, 8, 14);
register_bits!(init1, pre_ocd_x32, u8, 0, 3);

register!(init2, Init2, RW, u32);
register_bits!(init2, min_stable_clock_x1, u8, 0, 3);

register!(init3, Init3, RW, u32);
// DDR3/4: MR0
// LPDDR3/4: MR1
register_bits!(init3, mr, u16, 16, 31);
// DDR3/4: MR1
// Set bit 7 to 0.
// LPDDR3/4: MR2
register_bits!(init3, emr, u16, 0, 15);

register!(init4, Init4, RW, u32);
register_bits!(init4, emr2, u16, 16, 31);
register_bits!(init4, emr3, u16, 0, 15);

register!(init5, Init5, RW, u32);
register_bits!(init5, dev_zqinit_x32, u8, 16, 23);
register_bits!(init5, max_auto_init_x1024, u16, 0, 9);

register!(init6, Init6, RW, u32);
register_bits!(init6, mr4, u16, 16, 31);
register_bits!(init6, mr5, u16, 0, 15);

register!(init7, Init7, RW, u32);
register_bits!(init7, mr6, u16, 16, 31);

register!(dimm_ctrl, DimmCtrl, RW, u32);
register_bit!(dimm_ctrl, dimm_dis_bg_mirroring, 5);
register_bit!(dimm_ctrl, mrs_bg1_en, 4);
register_bit!(dimm_ctrl, mrs_a17_en, 3);
register_bit!(dimm_ctrl, dimm_output_inv_en, 2);
register_bit!(dimm_ctrl, dimm_addr_mirr_en, 1);
register_bit!(dimm_ctrl, dimm_stagger_cs_en, 0);

register!(dram_tmg0, DramTmg0, RW, u32);
register_bits!(dram_tmg0, wr2pre, u8, 24, 30);
register_bits!(dram_tmg0, t_faw, u8, 16, 21);
register_bits!(dram_tmg0, t_ras_max, u8, 8, 14);
register_bits!(dram_tmg0, t_ras_min, u8, 0, 5);

register!(dram_tmg1, DramTmg1, RW, u32);
register_bits!(dram_tmg1, t_xp, u8, 16, 20);
register_bits!(dram_tmg1, rd2pre, u8, 8, 12);
register_bits!(dram_tmg1, t_rc, u8, 0, 6);

register!(dram_tmg2, DramTmg2, RW, u32);
register_bits!(dram_tmg2, write_latency, u8, 24, 29);
register_bits!(dram_tmg2, read_latency, u8, 16, 21);
register_bits!(dram_tmg2, rd2wr, u8, 8, 13);
register_bits!(dram_tmg2, wr2rd, u8, 0, 5);

register!(dram_tmg3, DramTmg3, RW, u32);
register_bits!(dram_tmg3, t_mrw, u16, 20, 29);
register_bits!(dram_tmg3, t_mrd, u8, 12, 17);
register_bits!(dram_tmg3, t_mod, u16, 0, 9);

register!(dram_tmg4, DramTmg4, RW, u32);
register_bits!(dram_tmg4, t_rcd, u8, 24, 28);
register_bits!(dram_tmg4, t_ccd, u8, 16, 19);
register_bits!(dram_tmg4, t_rrd, u8, 8, 11);
register_bits!(dram_tmg4, t_rp, u8, 0, 4);

register!(dram_tmg8, DramTmg8, RW, u32);
register_bits!(dram_tmg8, t_xs_fast_x32, u8, 24, 30);
register_bits!(dram_tmg8, t_xs_abort_x32, u8, 16, 22);
register_bits!(dram_tmg8, t_xs_dll_x32, u8, 8, 14);
register_bits!(dram_tmg8, t_xs_x32, u8, 0, 6);

register!(dram_tmg9, DramTmg9, RW, u32);
register_bit!(dram_tmg9, ddr4_wr_preamble, 30);
register_bits!(dram_tmg9, t_ccd_s, u8, 16, 18);
register_bits!(dram_tmg9, t_rrd_s, u8, 8, 11);
register_bits!(dram_tmg9, wr2rd_s, u8, 0, 5);

register!(dram_tmg12, DramTmg12, RW, u32);
register_bits!(dram_tmg12, t_cmdcke, u8, 16, 17);
register_bits!(dram_tmg12, t_ckehcmd, u8, 8, 11);
register_bits!(dram_tmg12, t_mrd_pda, u8, 0, 4);

register!(addr_map0, AddrMap0, RW, u32);
register_bits!(addr_map0, addrmap_cs_bit0, u8, 0, 4);

register!(addr_map1, AddrMap1, RW, u32);
register_bits!(addr_map1, addrmap_bank_b2, u8, 16, 20);
register_bits!(addr_map1, addrmap_bank_b1, u8, 8, 12);
register_bits!(addr_map1, addrmap_bank_b0, u8, 0, 4);

register!(addr_map2, AddrMap2, RW, u32);
register_bits!(addr_map2, addrmap_col_b5, u8, 24, 27);
register_bits!(addr_map2, addrmap_col_b4, u8, 16, 19);
register_bits!(addr_map2, addrmap_col_b3, u8, 8, 11);
register_bits!(addr_map2, addrmap_col_b2, u8, 0, 3);

register!(addr_map3, AddrMap3, RW, u32);
register_bits!(addr_map3, addrmap_col_b9, u8, 24, 27);
register_bits!(addr_map3, addrmap_col_b8, u8, 16, 19);
register_bits!(addr_map3, addrmap_col_b7, u8, 8, 11);
register_bits!(addr_map3, addrmap_col_b6, u8, 0, 3);

register!(addr_map4, AddrMap4, RW, u32);
register_bits!(addr_map4, addrmap_col_b11, u8, 8, 11);
register_bits!(addr_map4, addrmap_col_b10, u8, 0, 3);

register!(addr_map5, AddrMap5, RW, u32);
register_bits!(addr_map5, addrmap_row_b11, u8, 24, 27);
register_bits!(addr_map5, addrmap_row_b2_10, u8, 16, 19);
register_bits!(addr_map5, addrmap_row_b1, u8, 8, 11);
register_bits!(addr_map5, addrmap_row_b0, u8, 0, 3);

register!(addr_map6, AddrMap6, RW, u32);
register_bit!(addr_map6, lpddr3_6gb_12gb, 31);
register_bits!(addr_map6, addrmap_row_b15, u8, 24, 27);
register_bits!(addr_map6, addrmap_row_b14, u8, 16, 19);
register_bits!(addr_map6, addrmap_row_b13, u8, 8, 11);
register_bits!(addr_map6, addrmap_row_b12, u8, 0, 3);

register!(addr_map7, AddrMap7, RW, u32);
register_bits!(addr_map7, addrmap_row_b17, u8, 8, 11);
register_bits!(addr_map7, addrmap_row_b16, u8, 0, 3);

register!(addr_map8, AddrMap8, RW, u32);
register_bits!(addr_map8, addrmap_bg_b1, u8, 8, 12);
register_bits!(addr_map8, addrmap_bg_b0, u8, 0, 4);

register!(addr_map9, AddrMap9, RW, u32);
register_bits!(addr_map9, addrmap_row_b5, u8, 24, 27);
register_bits!(addr_map9, addrmap_row_b4, u8, 16, 19);
register_bits!(addr_map9, addrmap_row_b3, u8, 8, 11);
register_bits!(addr_map9, addrmap_row_b2, u8, 0, 3);

register!(addr_map10, AddrMap10, RW, u32);
register_bits!(addr_map10, addrmap_row_b9, u8, 24, 27);
register_bits!(addr_map10, addrmap_row_b8, u8, 16, 19);
register_bits!(addr_map10, addrmap_row_b7, u8, 8, 11);
register_bits!(addr_map10, addrmap_row_b6, u8, 0, 3);

register!(addr_map11, AddrMap11, RW, u32);
register_bits!(addr_map11, addrmap_row_b10, u8, 0, 3);

register!(dq_map0, DqMap0, RW, u32);
register_bits!(dq_map0, dq_nibble_map_12_15, u8, 24, 31);
register_bits!(dq_map0, dq_nibble_map_8_11, u8, 16, 23);
register_bits!(dq_map0, dq_nibble_map_4_7, u8, 8, 15);
register_bits!(dq_map0, dq_nibble_map_0_3, u8, 0, 7);

register!(dq_map1, DqMap1, RW, u32);
register_bits!(dq_map1, dq_nibble_map_28_31, u8, 24, 31);
register_bits!(dq_map1, dq_nibble_map_24_27, u8, 16, 23);
register_bits!(dq_map1, dq_nibble_map_20_23, u8, 8, 15);
register_bits!(dq_map1, dq_nibble_map_16_19, u8, 0, 7);

register!(dq_map2, DqMap2, RW, u32);
register_bits!(dq_map2, dq_nibble_map_44_47, u8, 24, 31);
register_bits!(dq_map2, dq_nibble_map_40_43, u8, 16, 23);
register_bits!(dq_map2, dq_nibble_map_36_39, u8, 8, 15);
register_bits!(dq_map2, dq_nibble_map_32_35, u8, 0, 7);

register!(dq_map3, DqMap3, RW, u32);
register_bits!(dq_map3, dq_nibble_map_60_63, u8, 24, 31);
register_bits!(dq_map3, dq_nibble_map_56_59, u8, 16, 23);
register_bits!(dq_map3, dq_nibble_map_52_55, u8, 8, 15);
register_bits!(dq_map3, dq_nibble_map_48_51, u8, 0, 7);

register!(dq_map4, DqMap4, RW, u32);
register_bits!(dq_map4, dq_nibble_map_cb_4_7, u8, 8, 15);
register_bits!(dq_map4, dq_nibble_map_cb_0_3, u8, 0, 7);

register!(dq_map5, DqMap5, RW, u32);
register_bit!(dq_map5, dis_dq_rank_swap, 0);
