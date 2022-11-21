// Copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libboard_zynq/src/uart/baud_rate_gen.rs
// Original authors: Astro, Harry Ho, pca006132
// Modifications made for different clock sources, PLL configuration, and SLCRs

pub use super::slcr::crf_apb::ApuClkSource;
use super::slcr::{common::Unlocked, crf_apb, crl_apb};
use libregister::{RegisterR, RegisterRW};

pub mod source;
use source::*;

#[derive(Debug, Clone)]
pub struct Clocks {
    /// APU PLL: Recommended clock source for the APUs and the FPD interconnect
    pub apu: u32,
    /// DDR PLL: Recommended clock for the DDR DRAM controller and AXI_HP interfaces
    pub ddr: u32,
    /// Video PLL: Recommended clock for display port
    pub video: u32,
    /// I/O PLL: Recommended clock for I/O peripherals
    pub io: u32,
    /// RPU PLL: Recommended clock for RPUs and LPD interconnect
    pub rpu: u32,
}

impl Clocks {
    pub fn get() -> Self {
        let fpd_regs = crf_apb::RegisterBlock::slcr();
        let lpd_regs = crl_apb::RegisterBlock::slcr();
        Clocks {
            apu: ApuPll::freq(&mut fpd_regs.apu_pll_ctrl),
            ddr: DdrPll::freq(&mut fpd_regs.ddr_pll_ctrl),
            video: VideoPll::freq(&mut fpd_regs.video_pll_ctrl),
            io: IoPll::freq(&mut lpd_regs.io_pll_ctrl),
            rpu: RpuPll::freq(&mut lpd_regs.rpu_pll_ctrl),
        }
    }

    pub fn set_cpu_freq(target_freq: u32) {
        let fpd_regs = crf_apb::RegisterBlock::slcr();
        let apu_pll = ApuPll::freq(&mut fpd_regs.apu_pll_ctrl);
        let mut div = 1u8;
        while div < 63 && apu_pll / u32::from(div) > target_freq {
            div += 1;
        }

        crf_apb::RegisterBlock::unlocked(|slcr| {
            slcr.apu_clk_ctrl
                .modify(|_, w| w.srcsel(ApuClkSource::ApuPll).divisor0(div));
        })
    }

    pub fn uart0_ref_clk(&self) -> u32 {
        let lpd_regs = crl_apb::RegisterBlock::slcr();
        self.uart_ref_clk(&mut lpd_regs.uart0_clk_ctrl)
    }

    pub fn uart1_ref_clk(&self) -> u32 {
        let lpd_regs = crl_apb::RegisterBlock::slcr();
        self.uart_ref_clk(&mut lpd_regs.uart1_clk_ctrl)
    }

    fn uart_ref_clk(&self, uart_regs: &mut crl_apb::UartClkCtrl) -> u32 {
        let uart_clk_ctrl = uart_regs.read();
        let pll = match uart_clk_ctrl.srcsel() {
            crl_apb::IoClkSource::IoPll => self.io,
            crl_apb::IoClkSource::RpuPll => self.rpu,
            crl_apb::IoClkSource::DdrPllToLpd => {
                let fpd_regs = crf_apb::RegisterBlock::slcr();
                let divisor = u32::from(fpd_regs.ddr_pll_to_lpd_ctrl.read().divisor0());
                self.ddr / divisor
            }
        };
        pll / (u32::from(uart_clk_ctrl.divisor0()) * u32::from(uart_clk_ctrl.divisor1()))
    }
}
