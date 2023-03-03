///! FPD clock and reset control
use libregister::{
    register, register_at, register_bit, register_bits, register_bits_typed, RegisterW,
};
use volatile_register::{RO, RW, WO};

use super::common::{PllCfg, PllCtrl, PllFracCfg, Unlocked, WProt};

#[repr(u8)]
pub enum ApuClkSource {
    ApuPll = 0b00,
    DdrPll = 0b10,
    VideoPll = 0b11,
}
#[repr(C)]
pub struct RegisterBlock {
    // CRF_APB
    pub err_ctrl: RW<u32>,
    pub ir_status: RW<u32>, // todo: WTC LSB
    pub ir_mask: RO<u32>,
    pub ir_enable: WO<u32>,
    pub ir_disable: WO<u32>,
    unused0: [u32; 2],
    pub crf_wprot: WProt,
    pub apu_pll_ctrl: PllCtrl,
    pub apu_pll_cfg: PllCfg,
    pub apu_pll_frac_cfg: PllFracCfg,
    pub ddr_pll_ctrl: PllCtrl,
    pub ddr_pll_cfg: PllCfg,
    pub ddr_pll_frac_cfg: PllFracCfg,
    pub video_pll_ctrl: PllCtrl,
    pub video_pll_cfg: PllCfg,
    pub video_pll_frac_cfg: PllFracCfg,
    pub pll_status: PllStatus,
    pub apu_pll_to_lpd_ctrl: PllToLpdCtrl,
    pub ddr_pll_to_lpd_ctrl: PllToLpdCtrl,
    pub video_pll_to_lpd_ctrl: PllToLpdCtrl,
    unused1: [u32; 3],
    pub apu_clk_ctrl: ApuClkCtrl,
    pub dbg_trace_clk_ctrl: DbgTraceClkCtrl,
    pub dbg_fpd_clk_ctrl: DbgFpdClkCtrl,
    unused2: [u32; 1],
    pub dp_video_clk_ctrl: DpVideoClkCtrl,
    pub dp_audio_clk_ctrl: DpAudioClkCtrl,
    unused3: [u32; 1],
    pub dp_sys_clk_ctrl: DpSysClkCtrl,
    pub ddr_clk_ctrl: DdrClkCtrl,
    pub gpu_clk_ctrl: GpuClkCtrl,
    unused4: [u32; 6],
    pub sata_clk_ctrl: SataClkCtrl,
    unused5: [u32; 4],
    pub pcie_clk_ctrl: PcieClkCtrl,
    pub fpd_dma_clk_ctrl: FpdDmaClkCtrl,
    pub dp_dma_clk_ctrl: DpDmaClkCtrl,
    pub topsw_main_clk_ctrl: TopswMainClkCtrl,
    pub topsw_lsbus_clk_ctrl: TopswLsbusClkCtrl,
    unused6: [u32; 12],
    pub dbg_tstmp_clk_ctrl: DbgTimestampClkCtrl,
    unused7: [u32; 1],
    pub rst_fpd_top: RstFpdTop,
    pub rst_fpd_apu: RstFpdApu,
    pub rst_ddr_ss: RstDdrSS,
}
register_at!(RegisterBlock, 0xFD1A_0000, slcr);

impl Unlocked for RegisterBlock {
    fn unlocked<F: FnMut(&mut Self) -> R, R>(mut f: F) -> R {
        let mut self_ = Self::slcr();
        self_.crf_wprot.write(WProt::zeroed().active(false));
        let r = f(&mut self_);
        self_.crf_wprot.write(WProt::zeroed().active(true));
        r
    }
}

register!(pll_status, PllStatus, RO, u32);
register_bit!(pll_status, video_pll_stable, 5);
register_bit!(pll_status, ddr_pll_stable, 4);
register_bit!(pll_status, apu_pll_stable, 3);
register_bit!(pll_status, video_pll_lock, 2);
register_bit!(pll_status, ddr_pll_lock, 1);
register_bit!(pll_status, apu_pll_lock, 0);

register!(pll_to_lpd_ctrl, PllToLpdCtrl, RW, u32);
register_bits!(pll_to_lpd_ctrl, divisor0, u8, 8, 13);

register!(apu_clk_ctrl, ApuClkCtrl, RW, u32);
register_bit!(apu_clk_ctrl, clkact_half, 25);
register_bit!(apu_clk_ctrl, clkact_full, 24);
register_bits!(apu_clk_ctrl, divisor0, u8, 8, 13);
register_bits_typed!(apu_clk_ctrl, srcsel, u8, ApuClkSource, 0, 2);

register!(ddr_clk_ctrl, DdrClkCtrl, RW, u32);
register_bits!(ddr_clk_ctrl, divisor0, u8, 8, 13);
// 000: DDR PLL
// 001: Video PLL
register_bits!(ddr_clk_ctrl, srcsel, u8, 0, 2);

macro_rules! single_div_clk_reg {
    ($mod_name: ident, $struct_name: ident) => {
        register!($mod_name, $struct_name, RW, u32);
        register_bit!($mod_name, clkact, 24);
        register_bits!($mod_name, divisor0, u8, 8, 13);
        register_bits!($mod_name, srcsel, u8, 0, 2);
    };
}

single_div_clk_reg!(dbg_trace_clk_ctrl, DbgTraceClkCtrl);
single_div_clk_reg!(dbg_fpd_clk_ctrl, DbgFpdClkCtrl);
single_div_clk_reg!(gpu_clk_ctrl, GpuClkCtrl);
register_bit!(gpu_clk_ctrl, pp1_clkact, 26);
register_bit!(gpu_clk_ctrl, pp0_clkact, 25);
single_div_clk_reg!(sata_clk_ctrl, SataClkCtrl);
single_div_clk_reg!(pcie_clk_ctrl, PcieClkCtrl);
single_div_clk_reg!(fpd_dma_clk_ctrl, FpdDmaClkCtrl);
single_div_clk_reg!(dp_dma_clk_ctrl, DpDmaClkCtrl);
single_div_clk_reg!(topsw_main_clk_ctrl, TopswMainClkCtrl);
single_div_clk_reg!(topsw_lsbus_clk_ctrl, TopswLsbusClkCtrl);

macro_rules! dual_div_clk_reg {
    ($mod_name: ident, $struct_name: ident) => {
        register!($mod_name, $struct_name, RW, u32);
        register_bit!($mod_name, clkact, 24);
        register_bits!($mod_name, divisor1, u8, 16, 21);
        register_bits!($mod_name, divisor0, u8, 8, 13);
        register_bits!($mod_name, srcsel, u8, 0, 2);
    };
}

dual_div_clk_reg!(dp_video_clk_ctrl, DpVideoClkCtrl);
dual_div_clk_reg!(dp_audio_clk_ctrl, DpAudioClkCtrl);
dual_div_clk_reg!(dp_sys_clk_ctrl, DpSysClkCtrl);

register!(dbg_tstmp_clk_ctrl, DbgTimestampClkCtrl, RW, u32);
register_bits!(dbg_tstmp_clk_ctrl, divisor0, u8, 8, 13);
register_bits!(dbg_tstmp_clk_ctrl, srcsel, u8, 0, 2);

register!(rst_fpd_top, RstFpdTop, RW, u32);
register_bit!(rst_fpd_top, pcie_cfg_reset, 19);
register_bit!(rst_fpd_top, pcie_bridge_reset, 18);
register_bit!(rst_fpd_top, pcie_ctrl_reset, 17);
register_bit!(rst_fpd_top, dp_reset, 16);
register_bit!(rst_fpd_top, swdt_reset, 15);
register_bit!(rst_fpd_top, s_axi_hpc_3_fpd_reset, 12);
register_bit!(rst_fpd_top, s_axi_hpc_2_fpd_reset, 11);
register_bit!(rst_fpd_top, s_axi_hp_1_fpd_reset, 10);
register_bit!(rst_fpd_top, s_axi_hp_0_fpd_reset, 9);
register_bit!(rst_fpd_top, s_axi_hpc_1_fpd_reset, 8);
register_bit!(rst_fpd_top, s_axi_hpc_0_fpd_reset, 7);
register_bit!(rst_fpd_top, fpd_dma_reset, 6);
register_bit!(rst_fpd_top, gpu_pp1_reset, 5);
register_bit!(rst_fpd_top, gpu_pp0_reset, 4);
register_bit!(rst_fpd_top, gpu_reset, 3);
register_bit!(rst_fpd_top, gt_reset, 2);
register_bit!(rst_fpd_top, sata_reset, 1);

register!(rst_fpd_apu, RstFpdApu, RW, u32);
register_bit!(rst_fpd_apu, apu3_por, 13);
register_bit!(rst_fpd_apu, apu2_por, 12);
register_bit!(rst_fpd_apu, apu1_por, 11);
register_bit!(rst_fpd_apu, apu0_por, 10);
register_bit!(rst_fpd_apu, apu_l2_reset, 8);
register_bit!(rst_fpd_apu, apu3_reset, 3);
register_bit!(rst_fpd_apu, apu2_reset, 2);
register_bit!(rst_fpd_apu, apu1_reset, 1);
register_bit!(rst_fpd_apu, apu0_reset, 0);

register!(rst_ddr_ss, RstDdrSS, RW, u32);
register_bit!(rst_ddr_ss, ddr_reset, 3);
register_bit!(rst_ddr_ss, apm_reset, 2);
