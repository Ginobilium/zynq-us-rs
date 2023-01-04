use volatile_register::{RO, RW};

use libregister::{register, register_at, register_bit, register_bits_typed};

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DataBusWidth {
    Width32bit = 0b00,
    Width16bit = 0b01,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ControllerStatus {
    Init = 0,
    Normal = 1,
    Powerdown = 2,
    SelfRefresh = 3,
    Powerdown1 = 4,
    Powerdown2 = 5,
    Powerdown3 = 6,
    Powerdown4 = 7,
}

#[repr(u8)]
pub enum ZCalStatus {
    Success = 0b00,
    Overflow = 0b01,
    Underflow = 0b10,
    InProgress = 0b11,
}

#[repr(C)]
pub struct RegisterBlock {
    unused0: [u32; 1],
    pub phy_init: PhyInit,
    unused1: [u32; 2],
    pub pgcr0: RW<u32>,
    pub pgcr1: RW<u32>, // undocumented
    pub pgcr2: RW<u32>,
    pub pgcr3: RW<u32>,
    pub pgcr4: RW<u32>,
    pub pgcr5: RW<u32>,
    pub pgcr6: RW<u32>,
    pub pgcr7: RW<u32>,
    pub pgsr0: PGSR0,
    pub pgsr1: RO<u32>,
    pub pgsr2: RO<u32>,
    unused3: [u32; 1],
    pub ptr0: RW<u32>,
    pub ptr1: RW<u32>,
    pub ptr2: RW<u32>,
    pub ptr3: RW<u32>,
    pub ptr4: RW<u32>,
    pub ptr5: RW<u32>,
    pub ptr6: RW<u32>,
    unused4: [u32; 1],
    pub pll_ctrl0: RW<u32>,
    pub pll_ctrl1: RW<u32>,
    pub pll_ctrl2: RW<u32>,
    pub pll_ctrl3: RW<u32>,
    pub pll_ctrl4: RW<u32>,
    pub pll_ctrl5: RW<u32>,
    unused5: [u32; 2],
    pub dx_cfg: RW<u32>,
    unused6: [u32; 1],
    pub ddr_sys_cfg: RW<u32>,
    unused7: [u32; 1],
    pub odt_cfg: RW<u32>,
    unused8: [u32; 1],
    pub aa_ctrl: RW<u32>,
    unused9: [u32; 7],
    pub gpr0: RW<u32>,
    pub gpr1: RW<u32>,
    unused10: [u32; 14],
    pub dram_cfg: RW<u32>,
    unused11: [u32; 3],
    pub dtpr0: RW<u32>,
    pub dtpr1: RW<u32>,
    pub dtpr2: RW<u32>,
    pub dtpr3: RW<u32>,
    pub dtpr4: RW<u32>,
    pub dtpr5: RW<u32>,
    pub dtpr6: RW<u32>,
    unused12: [u32; 5],
    pub rdimm_cfg0: RW<u32>,
    pub rdimm_cfg1: RW<u32>,
    pub rdimm_cfg2: RW<u32>,
    unused13: [u32; 1],
    pub rdimm_ctrl0: RW<u32>,
    pub rdimm_ctrl1: RW<u32>,
    pub rdimm_ctrl2: RW<u32>,
    pub rdimm_ctrl3: RW<u32>,
    pub rdimm_ctrl4: RW<u32>,
    unused14: [u32; 1],
    pub sched_cmd0: RW<u32>,
    pub sched_cmd1: RW<u32>,
    unused15: [u32; 4],
    pub mode0: RW<u32>,
    pub mode1: RW<u32>,
    pub mode2: RW<u32>,
    pub mode3: RW<u32>,
    pub mode4: RW<u32>,
    pub mode5: RW<u32>,
    pub mode6: RW<u32>,
    pub mode7: RW<u32>,
    unused16: [u32; 3],
    pub mode11: RW<u32>,
    pub mode12: RW<u32>,
    pub mode13: RW<u32>,
    pub mode14: RW<u32>,
    unused17: [u32; 7],
    pub mode22: RW<u32>,
    unused18: [u32; 9],
    pub dt_cfg0: RW<u32>,
    pub dt_cfg1: RW<u32>,
    pub dt_addr0: RW<u32>,
    pub dt_addr1: RW<u32>,
    pub dt_addr2: RW<u32>,
    unused19: [u32; 1],
    pub dt_data0: RW<u32>,
    pub dt_data1: RW<u32>,
    unused20: [u32; 4],
    pub dt_eye_data0: RO<u32>,
    pub dt_eye_data1: RO<u32>,
    pub dt_eye_data2: RO<u32>,
    pub vt_data: RO<u32>,
    pub catr0: RW<u32>,
    pub catr1: RW<u32>,
    unused21: [u32; 2],
    pub dqs_drift0: RW<u32>,
    pub dqs_drift1: RW<u32>,
    pub dqs_drift2: RW<u32>,
    pub dt_eye_data3: RO<u32>, // 0x25c
    dont_care0: [u32; 165],
    pub rio_cr5: RW<u32>, // 0x4f4
    unused22: [u32; 2],
    pub aio_cr0: RW<u32>,
    pub aio_cr1: RW<u32>,
    pub aio_cr2: RW<u32>,
    pub aio_cr3: RW<u32>,
    pub aio_cr4: RW<u32>,
    pub aio_cr5: RW<u32>,
    unused23: [u32; 2],
    pub io_vcr0: RW<u32>,
    pub io_vcr1: RW<u32>,
    pub vtcr0: RW<u32>,
    pub vtcr1: RW<u32>,
    unused24: [u32; 4],
    pub ac_bdlr0: RW<u32>,
    pub ac_bdlr1: RW<u32>,
    pub ac_bdlr2: RW<u32>,
    pub ac_bdlr3: RW<u32>,
    pub ac_bdlr4: RW<u32>,
    pub ac_bdlr5: RW<u32>,
    pub ac_bdlr6: RW<u32>,
    pub ac_bdlr7: RW<u32>,
    pub ac_bdlr8: RW<u32>,
    pub ac_bdlr9: RW<u32>,
    unused25: [u32; 5],
    pub ac_bdlr15: RW<u32>,
    pub ac_bdlr16: RW<u32>,
    pub ac_lcdlr: RW<u32>,
    unused26: [u32; 6],
    pub ac_mdlr0: RW<u32>,
    pub ac_mdlr1: RW<u32>,
    unused27: [u32; 54],
    pub zqcr: RW<u32>,
    pub zq0_pr0: RW<u32>,
    pub zq0_pr1: RW<u32>,
    pub zq0_dr0: RW<u32>,
    pub zq0_dr1: RW<u32>,
    pub zq0_or0: RW<u32>,
    pub zq0_or1: RW<u32>,
    pub zq0_sr: ZQnSR, // 0x69c
    unused28: [u32; 1],
    pub zq1_pr0: RW<u32>,
    pub zq1_pr1: RW<u32>,
    pub zq1_dr0: RW<u32>,
    pub zq1_dr1: RW<u32>,
    pub zq1_or0: RW<u32>,
    pub zq1_or1: RW<u32>,
    pub zq1_sr: ZQnSR, // 0x6bc
    unused29: [u32; 16],
    pub dx0_gcr0: RW<u32>,
    pub dx0_gcr1: RW<u32>,
    pub dx0_gcr2: RW<u32>,
    pub dx0_gcr3: RW<u32>,
    pub dx0_gcr4: RW<u32>,
    pub dx0_gcr5: RW<u32>,
    pub dx0_gcr6: RW<u32>,
    unused30: [u32; 9],
    pub dx0_bdlr0: RW<u32>,
    pub dx0_bdlr1: RW<u32>,
    pub dx0_bdlr2: RW<u32>,
    unused31: [u32; 1],
    pub dx0_bdlr3: RW<u32>,
    pub dx0_bdlr4: RW<u32>,
    pub dx0_bdlr5: RW<u32>,
    unused32: [u32; 1],
    pub dx0_bdlr6: RW<u32>,
    unused33: [u32; 7],
    pub dx0_lcdlr0: RW<u32>,
    pub dx0_lcdlr1: RW<u32>,
    pub dx0_lcdlr2: RW<u32>,
    pub dx0_lcdlr3: RW<u32>,
    pub dx0_lcdlr4: RW<u32>,
    pub dx0_lcdlr5: RW<u32>, // 0x794
    unused34: [u32; 2],
    pub dx0_mdlr0: RW<u32>, // 0x7a0
    pub dx0_mdlr1: RW<u32>,
    unused35: [u32; 6],
    pub dx0_gtr0: RW<u32>, // 0x7c0
    unused36: [u32; 4],
    pub dx0_rsr1: RW<u32>,
    pub dx0_rsr2: RW<u32>,
    pub dx0_rsr3: RW<u32>,
    pub dx0_gsr0: DXnGSR0,
    pub dx0_gsr1: RW<u32>,
    pub dx0_gsr2: RW<u32>,
    pub dx0_gsr3: RW<u32>,
    unused37: [u32; 4],
    pub dx1_gcr0: RW<u32>,
    pub dx1_gcr1: RW<u32>,
    pub dx1_gcr2: RW<u32>,
    pub dx1_gcr3: RW<u32>,
    pub dx1_gcr4: RW<u32>,
    pub dx1_gcr5: RW<u32>,
    pub dx1_gcr6: RW<u32>,
    unused38: [u32; 9],
    pub dx1_bdlr0: RW<u32>,
    pub dx1_bdlr1: RW<u32>,
    pub dx1_bdlr2: RW<u32>,
    unused39: [u32; 1],
    pub dx1_bdlr3: RW<u32>,
    pub dx1_bdlr4: RW<u32>,
    pub dx1_bdlr5: RW<u32>,
    unused40: [u32; 1],
    pub dx1_bdlr6: RW<u32>,
    unused41: [u32; 7],
    pub dx1_lcdlr0: RW<u32>,
    pub dx1_lcdlr1: RW<u32>,
    pub dx1_lcdlr2: RW<u32>,
    pub dx1_lcdlr3: RW<u32>,
    pub dx1_lcdlr4: RW<u32>,
    pub dx1_lcdlr5: RW<u32>, // 0x794
    unused42: [u32; 2],
    pub dx1_mdlr0: RW<u32>, // 0x7a0
    pub dx1_mdlr1: RW<u32>,
    unused43: [u32; 6],
    pub dx1_gtr0: RW<u32>, // 0x7c0
    unused44: [u32; 4],
    pub dx1_rsr1: RW<u32>,
    pub dx1_rsr2: RW<u32>,
    pub dx1_rsr3: RW<u32>,
    pub dx1_gsr0: RW<u32>,
    pub dx1_gsr1: RW<u32>,
    pub dx1_gsr2: RW<u32>,
    pub dx1_gsr3: RW<u32>,
    unused45: [u32; 4],
    pub dx2_gcr0: RW<u32>,
    pub dx2_gcr1: RW<u32>,
    pub dx2_gcr2: RW<u32>,
    pub dx2_gcr3: RW<u32>,
    pub dx2_gcr4: RW<u32>,
    pub dx2_gcr5: RW<u32>,
    pub dx2_gcr6: RW<u32>,
    unused46: [u32; 9],
    pub dx2_bdlr0: RW<u32>,
    pub dx2_bdlr1: RW<u32>,
    pub dx2_bdlr2: RW<u32>,
    unused47: [u32; 1],
    pub dx2_bdlr3: RW<u32>,
    pub dx2_bdlr4: RW<u32>,
    pub dx2_bdlr5: RW<u32>,
    unused48: [u32; 1],
    pub dx2_bdlr6: RW<u32>,
    unused49: [u32; 7],
    pub dx2_lcdlr0: RW<u32>,
    pub dx2_lcdlr1: RW<u32>,
    pub dx2_lcdlr2: RW<u32>,
    pub dx2_lcdlr3: RW<u32>,
    pub dx2_lcdlr4: RW<u32>,
    pub dx2_lcdlr5: RW<u32>, // 0x794
    unused50: [u32; 2],
    pub dx2_mdlr0: RW<u32>, // 0x7a0
    pub dx2_mdlr1: RW<u32>,
    unused51: [u32; 6],
    pub dx2_gtr0: RW<u32>, // 0x7c0
    unused52: [u32; 4],
    pub dx2_rsr1: RW<u32>,
    pub dx2_rsr2: RW<u32>,
    pub dx2_rsr3: RW<u32>,
    pub dx2_gsr0: DXnGSR0,
    pub dx2_gsr1: RW<u32>,
    pub dx2_gsr2: RW<u32>,
    pub dx2_gsr3: RW<u32>,
    unused53: [u32; 4],
    pub dx3_gcr0: RW<u32>,
    pub dx3_gcr1: RW<u32>,
    pub dx3_gcr2: RW<u32>,
    pub dx3_gcr3: RW<u32>,
    pub dx3_gcr4: RW<u32>,
    pub dx3_gcr5: RW<u32>,
    pub dx3_gcr6: RW<u32>,
    unused54: [u32; 9],
    pub dx3_bdlr0: RW<u32>,
    pub dx3_bdlr1: RW<u32>,
    pub dx3_bdlr2: RW<u32>,
    unused55: [u32; 1],
    pub dx3_bdlr3: RW<u32>,
    pub dx3_bdlr4: RW<u32>,
    pub dx3_bdlr5: RW<u32>,
    unused56: [u32; 1],
    pub dx3_bdlr6: RW<u32>,
    unused57: [u32; 7],
    pub dx3_lcdlr0: RW<u32>,
    pub dx3_lcdlr1: RW<u32>,
    pub dx3_lcdlr2: RW<u32>,
    pub dx3_lcdlr3: RW<u32>,
    pub dx3_lcdlr4: RW<u32>,
    pub dx3_lcdlr5: RW<u32>, // 0x794
    unused58: [u32; 2],
    pub dx3_mdlr0: RW<u32>, // 0x7a0
    pub dx3_mdlr1: RW<u32>,
    unused59: [u32; 6],
    pub dx3_gtr0: RW<u32>, // 0x7c0
    unused60: [u32; 4],
    pub dx3_rsr1: RW<u32>,
    pub dx3_rsr2: RW<u32>,
    pub dx3_rsr3: RW<u32>,
    pub dx3_gsr0: RW<u32>,
    pub dx3_gsr1: RW<u32>,
    pub dx3_gsr2: RW<u32>,
    pub dx3_gsr3: RW<u32>,
    unused61: [u32; 4],
    pub dx4_gcr0: RW<u32>,
    pub dx4_gcr1: RW<u32>,
    pub dx4_gcr2: RW<u32>,
    pub dx4_gcr3: RW<u32>,
    pub dx4_gcr4: RW<u32>,
    pub dx4_gcr5: RW<u32>,
    pub dx4_gcr6: RW<u32>,
    unused62: [u32; 9],
    pub dx4_bdlr0: RW<u32>,
    pub dx4_bdlr1: RW<u32>,
    pub dx4_bdlr2: RW<u32>,
    unused63: [u32; 1],
    pub dx4_bdlr3: RW<u32>,
    pub dx4_bdlr4: RW<u32>,
    pub dx4_bdlr5: RW<u32>,
    unused64: [u32; 1],
    pub dx4_bdlr6: RW<u32>,
    unused65: [u32; 7],
    pub dx4_lcdlr0: RW<u32>,
    pub dx4_lcdlr1: RW<u32>,
    pub dx4_lcdlr2: RW<u32>,
    pub dx4_lcdlr3: RW<u32>,
    pub dx4_lcdlr4: RW<u32>,
    pub dx4_lcdlr5: RW<u32>, // 0x794
    unused66: [u32; 2],
    pub dx4_mdlr0: RW<u32>, // 0x7a0
    pub dx4_mdlr1: RW<u32>,
    unused67: [u32; 6],
    pub dx4_gtr0: RW<u32>, // 0x7c0
    unused68: [u32; 4],
    pub dx4_rsr1: RW<u32>,
    pub dx4_rsr2: RW<u32>,
    pub dx4_rsr3: RW<u32>,
    pub dx4_gsr0: DXnGSR0,
    pub dx4_gsr1: RW<u32>,
    pub dx4_gsr2: RW<u32>,
    pub dx4_gsr3: RW<u32>,
    unused69: [u32; 4],
    pub dx5_gcr0: RW<u32>,
    pub dx5_gcr1: RW<u32>,
    pub dx5_gcr2: RW<u32>,
    pub dx5_gcr3: RW<u32>,
    pub dx5_gcr4: RW<u32>,
    pub dx5_gcr5: RW<u32>,
    pub dx5_gcr6: RW<u32>,
    unused70: [u32; 9],
    pub dx5_bdlr0: RW<u32>,
    pub dx5_bdlr1: RW<u32>,
    pub dx5_bdlr2: RW<u32>,
    unused71: [u32; 1],
    pub dx5_bdlr3: RW<u32>,
    pub dx5_bdlr4: RW<u32>,
    pub dx5_bdlr5: RW<u32>,
    unused72: [u32; 1],
    pub dx5_bdlr6: RW<u32>,
    unused73: [u32; 7],
    pub dx5_lcdlr0: RW<u32>,
    pub dx5_lcdlr1: RW<u32>,
    pub dx5_lcdlr2: RW<u32>,
    pub dx5_lcdlr3: RW<u32>,
    pub dx5_lcdlr4: RW<u32>,
    pub dx5_lcdlr5: RW<u32>, // 0x794
    unused74: [u32; 2],
    pub dx5_mdlr0: RW<u32>, // 0x7a0
    pub dx5_mdlr1: RW<u32>,
    unused75: [u32; 6],
    pub dx5_gtr0: RW<u32>, // 0x7c0
    unused76: [u32; 4],
    pub dx5_rsr1: RW<u32>,
    pub dx5_rsr2: RW<u32>,
    pub dx5_rsr3: RW<u32>,
    pub dx5_gsr0: RW<u32>,
    pub dx5_gsr1: RW<u32>,
    pub dx5_gsr2: RW<u32>,
    pub dx5_gsr3: RW<u32>,
    unused77: [u32; 4],
    pub dx6_gcr0: RW<u32>,
    pub dx6_gcr1: RW<u32>,
    pub dx6_gcr2: RW<u32>,
    pub dx6_gcr3: RW<u32>,
    pub dx6_gcr4: RW<u32>,
    pub dx6_gcr5: RW<u32>,
    pub dx6_gcr6: RW<u32>,
    unused78: [u32; 9],
    pub dx6_bdlr0: RW<u32>,
    pub dx6_bdlr1: RW<u32>,
    pub dx6_bdlr2: RW<u32>,
    unused79: [u32; 1],
    pub dx6_bdlr3: RW<u32>,
    pub dx6_bdlr4: RW<u32>,
    pub dx6_bdlr5: RW<u32>,
    unused80: [u32; 1],
    pub dx6_bdlr6: RW<u32>,
    unused81: [u32; 7],
    pub dx6_lcdlr0: RW<u32>,
    pub dx6_lcdlr1: RW<u32>,
    pub dx6_lcdlr2: RW<u32>,
    pub dx6_lcdlr3: RW<u32>,
    pub dx6_lcdlr4: RW<u32>,
    pub dx6_lcdlr5: RW<u32>, // 0x794
    unused82: [u32; 2],
    pub dx6_mdlr0: RW<u32>, // 0x7a0
    pub dx6_mdlr1: RW<u32>,
    unused83: [u32; 6],
    pub dx6_gtr0: RW<u32>, // 0x7c0
    unused84: [u32; 4],
    pub dx6_rsr1: RW<u32>,
    pub dx6_rsr2: RW<u32>,
    pub dx6_rsr3: RW<u32>,
    pub dx6_gsr0: DXnGSR0,
    pub dx6_gsr1: RW<u32>,
    pub dx6_gsr2: RW<u32>,
    pub dx6_gsr3: RW<u32>,
    unused85: [u32; 4],
    pub dx7_gcr0: RW<u32>,
    pub dx7_gcr1: RW<u32>,
    pub dx7_gcr2: RW<u32>,
    pub dx7_gcr3: RW<u32>,
    pub dx7_gcr4: RW<u32>,
    pub dx7_gcr5: RW<u32>,
    pub dx7_gcr6: RW<u32>,
    unused86: [u32; 9],
    pub dx7_bdlr0: RW<u32>,
    pub dx7_bdlr1: RW<u32>,
    pub dx7_bdlr2: RW<u32>,
    unused87: [u32; 1],
    pub dx7_bdlr3: RW<u32>,
    pub dx7_bdlr4: RW<u32>,
    pub dx7_bdlr5: RW<u32>,
    unused88: [u32; 1],
    pub dx7_bdlr6: RW<u32>,
    unused89: [u32; 7],
    pub dx7_lcdlr0: RW<u32>,
    pub dx7_lcdlr1: RW<u32>,
    pub dx7_lcdlr2: RW<u32>,
    pub dx7_lcdlr3: RW<u32>,
    pub dx7_lcdlr4: RW<u32>,
    pub dx7_lcdlr5: RW<u32>, // 0x794
    unused90: [u32; 2],
    pub dx7_mdlr0: RW<u32>, // 0x7a0
    pub dx7_mdlr1: RW<u32>,
    unused91: [u32; 6],
    pub dx7_gtr0: RW<u32>, // 0x7c0
    unused92: [u32; 4],
    pub dx7_rsr1: RW<u32>,
    pub dx7_rsr2: RW<u32>,
    pub dx7_rsr3: RW<u32>,
    pub dx7_gsr0: RW<u32>,
    pub dx7_gsr1: RW<u32>,
    pub dx7_gsr2: RW<u32>,
    pub dx7_gsr3: RW<u32>,
    unused93: [u32; 4],
    pub dx8_gcr0: RW<u32>,
    pub dx8_gcr1: RW<u32>,
    pub dx8_gcr2: RW<u32>,
    pub dx8_gcr3: RW<u32>,
    pub dx8_gcr4: RW<u32>,
    pub dx8_gcr5: RW<u32>,
    pub dx8_gcr6: RW<u32>,
    unused94: [u32; 9],
    pub dx8_bdlr0: RW<u32>,
    pub dx8_bdlr1: RW<u32>,
    pub dx8_bdlr2: RW<u32>,
    unused95: [u32; 1],
    pub dx8_bdlr3: RW<u32>,
    pub dx8_bdlr4: RW<u32>,
    pub dx8_bdlr5: RW<u32>,
    unused96: [u32; 1],
    pub dx8_bdlr6: RW<u32>,
    unused97: [u32; 7],
    pub dx8_lcdlr0: RW<u32>,
    pub dx8_lcdlr1: RW<u32>,
    pub dx8_lcdlr2: RW<u32>,
    pub dx8_lcdlr3: RW<u32>,
    pub dx8_lcdlr4: RW<u32>,
    pub dx8_lcdlr5: RW<u32>, // 0x794
    unused98: [u32; 2],
    pub dx8_mdlr0: RW<u32>, // 0x7a0
    pub dx8_mdlr1: RW<u32>,
    unused99: [u32; 6],
    pub dx8_gtr0: RW<u32>, // 0x7c0
    unused100: [u32; 4],
    pub dx8_rsr1: RW<u32>,
    pub dx8_rsr2: RW<u32>,
    pub dx8_rsr3: RW<u32>,
    pub dx8_gsr0: DXnGSR0,
    pub dx8_gsr1: RW<u32>,
    pub dx8_gsr2: RW<u32>,
    pub dx8_gsr3: RW<u32>,
    unused101: [u32; 260],
    pub dx8_sl0_osc: RW<u32>,
    pub dx8_sl0_pllcr0: RW<u32>,
    pub dx8_sl0_pllcr1: RW<u32>,
    pub dx8_sl0_pllcr2: RW<u32>,
    pub dx8_sl0_pllcr3: RW<u32>,
    pub dx8_sl0_pllcr4: RW<u32>,
    pub dx8_sl0_pllcr5: RW<u32>,
    pub dx8_sl0_dqsctl: RW<u32>,
    pub dx8_sl0_trnctl: RW<u32>,
    pub dx8_sl0_ddlctl: RW<u32>,
    pub dx8_sl0_dxctl1: RW<u32>,
    pub dx8_sl0_dxctl2: RW<u32>,
    pub dx8_sl0_iocr: RW<u32>,
    unused102: [u32; 3],
    pub dx8_sl1_osc: RW<u32>,
    pub dx8_sl1_pllcr0: RW<u32>,
    pub dx8_sl1_pllcr1: RW<u32>,
    pub dx8_sl1_pllcr2: RW<u32>,
    pub dx8_sl1_pllcr3: RW<u32>,
    pub dx8_sl1_pllcr4: RW<u32>,
    pub dx8_sl1_pllcr5: RW<u32>,
    pub dx8_sl1_dqsctl: RW<u32>,
    pub dx8_sl1_trnctl: RW<u32>,
    pub dx8_sl1_ddlctl: RW<u32>,
    pub dx8_sl1_dxctl1: RW<u32>,
    pub dx8_sl1_dxctl2: RW<u32>,
    pub dx8_sl1_iocr: RW<u32>,
    unused103: [u32; 3],
    pub dx8_sl2_osc: RW<u32>,
    pub dx8_sl2_pllcr0: RW<u32>,
    pub dx8_sl2_pllcr1: RW<u32>,
    pub dx8_sl2_pllcr2: RW<u32>,
    pub dx8_sl2_pllcr3: RW<u32>,
    pub dx8_sl2_pllcr4: RW<u32>,
    pub dx8_sl2_pllcr5: RW<u32>,
    pub dx8_sl2_dqsctl: RW<u32>,
    pub dx8_sl2_trnctl: RW<u32>,
    pub dx8_sl2_ddlctl: RW<u32>,
    pub dx8_sl2_dxctl1: RW<u32>,
    pub dx8_sl2_dxctl2: RW<u32>,
    pub dx8_sl2_iocr: RW<u32>,
    unused104: [u32; 3],
    pub dx8_sl3_osc: RW<u32>,
    pub dx8_sl3_pllcr0: RW<u32>,
    pub dx8_sl3_pllcr1: RW<u32>,
    pub dx8_sl3_pllcr2: RW<u32>,
    pub dx8_sl3_pllcr3: RW<u32>,
    pub dx8_sl3_pllcr4: RW<u32>,
    pub dx8_sl3_pllcr5: RW<u32>,
    pub dx8_sl3_dqsctl: RW<u32>,
    pub dx8_sl3_trnctl: RW<u32>,
    pub dx8_sl3_ddlctl: RW<u32>,
    pub dx8_sl3_dxctl1: RW<u32>,
    pub dx8_sl3_dxctl2: RW<u32>,
    pub dx8_sl3_iocr: RW<u32>,
    unused105: [u32; 3],
    pub dx8_sl4_osc: RW<u32>,
    pub dx8_sl4_pllcr0: RW<u32>,
    pub dx8_sl4_pllcr1: RW<u32>,
    pub dx8_sl4_pllcr2: RW<u32>,
    pub dx8_sl4_pllcr3: RW<u32>,
    pub dx8_sl4_pllcr4: RW<u32>,
    pub dx8_sl4_pllcr5: RW<u32>,
    pub dx8_sl4_dqsctl: RW<u32>,
    pub dx8_sl4_trnctl: RW<u32>,
    pub dx8_sl4_ddlctl: RW<u32>,
    pub dx8_sl4_dxctl1: RW<u32>,
    pub dx8_sl4_dxctl2: RW<u32>,
    pub dx8_sl4_iocr: RW<u32>,
    unused106: [u32; 163],
    pub dx8_slb_osc: RW<u32>,
    pub dx8_slb_pllcr0: RW<u32>,
    pub dx8_slb_pllcr1: RW<u32>,
    pub dx8_slb_pllcr2: RW<u32>,
    pub dx8_slb_pllcr3: RW<u32>,
    pub dx8_slb_pllcr4: RW<u32>,
    pub dx8_slb_pllcr5: RW<u32>,
    pub dx8_slb_dqsctl: RW<u32>,
    pub dx8_slb_trnctl: RW<u32>,
    pub dx8_slb_ddlctl: RW<u32>,
    pub dx8_slb_dxctl1: RW<u32>,
    pub dx8_slb_dxctl2: RW<u32>,
    pub dx8_slb_iocr: RW<u32>,
}

register_at!(RegisterBlock, 0xFD08_0000, ddr_phy);

// PIR
register!(phy_init, PhyInit, RW, u32);
register_bit!(phy_init, zcal_bypass, 30, WTC);
register_bit!(phy_init, delay_cal_pause, 29, WTC);
register_bit!(phy_init, dqs2dq, 20);
register_bit!(phy_init, rdimm_init, 19);
register_bit!(phy_init, ctrl_dram_init, 18);
register_bit!(phy_init, vref, 17);
register_bit!(phy_init, wr_eye, 15);
register_bit!(phy_init, rd_eye, 14);
register_bit!(phy_init, wr_deskew, 13);
register_bit!(phy_init, rd_deskew, 12);
register_bit!(phy_init, wr_lev_adj, 11);
register_bit!(phy_init, qs_gate, 10);
register_bit!(phy_init, wr_lev, 9);
register_bit!(phy_init, dram_init, 8);
register_bit!(phy_init, dram_rst, 7);
register_bit!(phy_init, phy_rst, 6);
register_bit!(phy_init, dcal, 5);
register_bit!(phy_init, pll_init, 4);
register_bit!(phy_init, lpddr3_ca, 2);
register_bit!(phy_init, zcal, 1);
register_bit!(phy_init, init, 0, WTC);

register!(pgsr0, PGSR0, RO, u32);
register_bit!(pgsr0, ap_lock, 31);
register_bit!(pgsr0, ca_wrn, 29);
register_bit!(pgsr0, ca_err, 28);
register_bit!(pgsr0, we_err, 27);
register_bit!(pgsr0, re_err, 26);
register_bit!(pgsr0, wd_err, 25);
register_bit!(pgsr0, rd_err, 24);
register_bit!(pgsr0, wla_err, 23);
register_bit!(pgsr0, qsg_err, 22);
register_bit!(pgsr0, wl_err, 21);
register_bit!(pgsr0, zc_err, 20);
register_bit!(pgsr0, v_err, 19);
register_bit!(pgsr0, dqs2dq_err, 18);
register_bit!(pgsr0, dqs2dq_done, 15);
register_bit!(pgsr0, v_done, 14);
register_bit!(pgsr0, ca_done, 12);
register_bit!(pgsr0, we_done, 11); // no we are not
register_bit!(pgsr0, re_done, 10);
register_bit!(pgsr0, wd_done, 9);
register_bit!(pgsr0, rd_done, 8);
register_bit!(pgsr0, wla_done, 7);
register_bit!(pgsr0, qsg_done, 6);
register_bit!(pgsr0, wl_done, 5);
register_bit!(pgsr0, di_done, 4);
register_bit!(pgsr0, zc_done, 3);
register_bit!(pgsr0, dc_done, 2);
register_bit!(pgsr0, pl_done, 1);
// Wait at least 32 ctl_clk cycles after this is first observed to be 1b1
// before starting/resuming traffic to DRAM
// or triggering new PIR.INIT
register_bit!(pgsr0, i_done, 0);

register!(zqnsr, ZQnSR, RO, u32);
register_bit!(zqnsr, pd_odt_sat, 13);
register_bit!(zqnsr, pu_odt_sat, 12);
register_bit!(zqnsr, pd_drv_sat, 11);
register_bit!(zqnsr, pu_drv_sat, 10);
register_bit!(zqnsr, z_done, 9);
register_bit!(zqnsr, z_err, 8);
register_bits_typed!(zqnsr, opu, u8, ZCalStatus, 6, 7);
register_bits_typed!(zqnsr, opd, u8, ZCalStatus, 4, 5);
register_bits_typed!(zqnsr, zpu, u8, ZCalStatus, 2, 3);
register_bits_typed!(zqnsr, zpd, u8, ZCalStatus, 2, 3);

// even values of n
register!(dxn_gsr0, DXnGSR0, RO, u32);
register_bit!(dxn_gsr0, dp_lock, 16);
// other bits excluded for now