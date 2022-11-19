///! FPD clock and reset control

use volatile_register::{RO, RW, WO};
use libregister::{
    register, register_at,
    register_bit, register_bits, register_bits_typed,
    RegisterW,
};

use super::common::{Unlocked, WProt, PllCfg, PllCtrl, PllFracCfg};

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
    pub ir_status: RW<u32>,  // todo: WTC LSB
    pub ir_mask: RO<u32>,
    pub ir_enable: WO<u32>,
    pub ir_disable: WO<u32>,
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
    pub dbg_trace_clk_ctrl: RW<u32>,
    pub dbg_fpd_clk_ctrl: RW<u32>,
    unused2: [u32; 1],
    pub dp_video_clk_ctrl: RW<u32>,
    pub dp_audio_clk_ctrl: RW<u32>,
    unused3: [u32; 1],
    pub dp_sys_clk_ctrl: RW<u32>,
    pub ddr_clk_ctrl: DdrClkCtrl,
    pub gpu_clk_ctrl: RW<u32>,
    unused4: [u32; 6],
    pub sata_clk_ctrl: RW<u32>,
    unused5: [u32; 4],
    pub pcie_clk_ctrl: RW<u32>,
    pub fpd_dma_clk_ctrl: RW<u32>,
    pub dp_dma_clk_ctrl: RW<u32>,
    pub topsw_main_clk_ctrl: RW<u32>,
    pub topsw_lsbus_clk_ctrl: RW<u32>,
    unused6: [u32; 8],
    pub dbg_tstmp_clk_ctrl: RW<u32>,
    unused7: [u32; 1],
    pub rst_fpd_top: RW<u32>,
    pub rst_fpd_apu: RW<u32>,
    pub rst_ddr_ss: RW<u32>,
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
