// Copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libboard_zynq/src/uart/baud_rate_gen.rs
// Original authors: Astro, Harry Ho, pca006132
// Modifications made for different clock sources, PLL configuration, and SLCRs

use super::slcr::{
    common::Unlocked,
    crf_apb::{self, ApuClkSource},
    crl_apb::{self, IoClkSource, RpuClkSource},
    iou_slcr,
};
use libregister::{RegisterR, RegisterW};

pub mod source;
use source::*;

// DS926 Table 38: PS Clocks Switching Characteristics
#[allow(unused)]
const TOP_SW_MAIN_MAX_FREQ: u32 = 533_000_000;
#[allow(unused)]
const TOP_SW_LSBUS_MAX_FREQ: u32 = 100_000_000;
#[allow(unused)]
const FPD_DMA_MAX_FREQ: u32 = 600_000_000;
#[allow(unused)]
const DP_DMA_MAX_FREQ: u32 = 600_000_000;
#[allow(unused)]
const LPD_SWITCH_MAX_FREQ: u32 = 500_000_000;
#[allow(unused)]
const LPD_LSBUS_MAX_FREQ: u32 = 100_000_000;
// same for all the PLL_TO_(F|L)PDs
#[allow(unused)]
const XDOMAIN_MAX_FREQ: u32 = 533_000_000;
// max PCAP freq is dependent on Vccint, see DS926 Table 26
// for ZCU111 (ZU28DR -2E) Vccint = 0.85 V
#[allow(unused)]
#[cfg(feature = "target_zcu111")]
const PCAP_MAX_FREQ: u32 = 200_000_000;

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
    /// Initialize PLLs and component clock sources
    pub fn init() {
        init_plls();
        Self::init_xdomain_clocks();
        Self::init_lpd_clocks();
        Self::init_fpd_clocks();
        Self::init_misc_clocks();
    }

    pub fn get() -> Self {
        let fpd_regs = crf_apb::RegisterBlock::crf_apb();
        let lpd_regs = crl_apb::RegisterBlock::crl_apb();
        Clocks {
            apu: ApuPll::freq(&mut fpd_regs.apu_pll_ctrl),
            ddr: DdrPll::freq(&mut fpd_regs.ddr_pll_ctrl),
            video: VideoPll::freq(&mut fpd_regs.video_pll_ctrl),
            io: IoPll::freq(&mut lpd_regs.io_pll_ctrl),
            rpu: RpuPll::freq(&mut lpd_regs.rpu_pll_ctrl),
        }
    }

    fn init_xdomain_clocks() {
        // divisors for each PLL to the other power domain
        let rpll_div0: u8 = 2; // 500 MHz
        let iopll_div0: u8 = 3; // 500 MHz
        let apll_div0: u8 = 3; // 400 MHz
        let dpll_div0: u8 = 2; // 533 MHz
        let vpll_div0: u8 = 3; // 500 MHz

        crl_apb::RegisterBlock::unlocked(|crl_apb| {
            crl_apb
                .rpu_pll_to_fpd_ctrl
                .write(crl_apb::PllToFpdCtrl::zeroed().divisor0(rpll_div0));
            crl_apb
                .io_pll_to_fpd_ctrl
                .write(crl_apb::PllToFpdCtrl::zeroed().divisor0(iopll_div0));
        });

        crf_apb::RegisterBlock::unlocked(|crf_apb| {
            crf_apb
                .apu_pll_to_lpd_ctrl
                .write(crf_apb::PllToLpdCtrl::zeroed().divisor0(apll_div0));
            crf_apb
                .ddr_pll_to_lpd_ctrl
                .write(crf_apb::PllToLpdCtrl::zeroed().divisor0(dpll_div0));
            crf_apb
                .video_pll_to_lpd_ctrl
                .write(crf_apb::PllToLpdCtrl::zeroed().divisor0(vpll_div0));
        });
    }

    fn init_lpd_clocks() {
        // Initialize clock sources and divisors for LPD components
        crl_apb::RegisterBlock::unlocked(|crl_apb| {
            // Source: IO PLL
            // 125 MHz
            crl_apb.gem3_clk_ctrl.write(
                crl_apb::GemClkCtrl::zeroed()
                    .rx_clkact(true)
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(12)
                    .srcsel(IoClkSource::IoPll),
            );
            // 250 MHz
            crl_apb.gem_tsu_clk_ctrl.write(
                crl_apb::GemTsuClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(6)
                    .srcsel(IoClkSource::IoPll),
            );
            // 250 MHz
            crl_apb.usb0_bus_clk_ctrl.write(
                crl_apb::UsbClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(6)
                    .srcsel(IoClkSource::IoPll),
            );
            // 20 MHz
            crl_apb.usb3_clk_ctrl.write(
                crl_apb::UsbClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(3)
                    .divisor0(25)
                    .srcsel(IoClkSource::IoPll),
            );
            // 125 MHz
            crl_apb.qspi_clk_ctrl.write(
                crl_apb::QSpiClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(12)
                    .srcsel(IoClkSource::IoPll),
            );
            // 187.5 MHz
            crl_apb.sdio1_clk_ctrl.write(
                crl_apb::SdioClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(8)
                    .srcsel(IoClkSource::IoPll),
            );
            // 50 MHz
            crl_apb.uart0_clk_ctrl.write(
                crl_apb::UartClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(30)
                    .srcsel(IoClkSource::IoPll),
            );
            // 50 MHz
            crl_apb.uart1_clk_ctrl.write(
                crl_apb::UartClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(30)
                    .srcsel(IoClkSource::IoPll),
            );
            // 100 MHz
            crl_apb.i2c0_clk_ctrl.write(
                crl_apb::I2cClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(15)
                    .srcsel(IoClkSource::IoPll),
            );
            // 100 MHz
            crl_apb.i2c1_clk_ctrl.write(
                crl_apb::I2cClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(15)
                    .srcsel(IoClkSource::IoPll),
            );
            // 187.5 MHz
            crl_apb.pcap_clk_ctrl.write(
                crl_apb::PcapClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(8)
                    .srcsel(IoClkSource::IoPll),
            );
            // 100 MHz
            crl_apb.pl0_clk_ctrl.write(
                crl_apb::PlClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(15)
                    .srcsel(IoClkSource::IoPll),
            );
            // IO PLL
            crl_apb.dll_clk_ctrl.write(crl_apb::DllClkCtrl::zeroed());
            // 100 MHz
            crl_apb.timestamp_clk_ctrl.write(
                crl_apb::TimestampClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(15)
                    .srcsel(IoClkSource::IoPll),
            );

            // Source: RPU PLL
            // 500 MHz
            // Also the clock for OCM
            crl_apb.rpu_clk_ctrl.write(
                crl_apb::RpuClkCtrl::zeroed()
                    // .clkact_core(true)
                    .clkact(true)
                    .divisor0(2)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 250 MHz
            crl_apb.iou_switch_clk_ctrl.write(
                crl_apb::IouSwitchClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(4)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 500 MHz
            crl_apb.lpd_switch_clk_ctrl.write(
                crl_apb::LpdSwitchClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(2)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 100 MHz
            crl_apb.lpd_lsbus_clk_ctrl.write(
                crl_apb::LpdLsbusClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(10)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 250 MHz
            crl_apb.dbg_lpd_clk_ctrl.write(
                crl_apb::DbgLpdClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(4)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 500 MHz
            crl_apb.lpd_dma_clk_ctrl.write(
                crl_apb::LpdDmaClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(2)
                    .srcsel(RpuClkSource::RpuPll),
            );
            // 50 MHz
            crl_apb.ps_sysmon_clk_ctrl.write(
                crl_apb::PsSysmonClkCtrl::zeroed()
                    .clkact(true)
                    .divisor1(1)
                    .divisor0(20)
                    .srcsel(RpuClkSource::RpuPll),
            );
        });
    }

    fn init_fpd_clocks() {
        crf_apb::RegisterBlock::unlocked(|crf_apb| {
            // Source: APU PLL
            // 1200 MHz
            crf_apb.apu_clk_ctrl.write(
                crf_apb::ApuClkCtrl::zeroed()
                    .clkact_half(true)
                    .clkact_full(true)
                    .divisor0(1)
                    .srcsel(ApuClkSource::ApuPll),
            );

            // Source: DDR PLL
            // 533 MHz
            crf_apb
                .ddr_clk_ctrl
                .write(crf_apb::DdrClkCtrl::zeroed().divisor0(2).srcsel(0));
            // 533 MHz
            crf_apb.topsw_main_clk_ctrl.write(
                crf_apb::TopswMainClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(2)
                    .srcsel(3),
            );

            // Source: IO PLL to FPD
            // 100 MHz
            crf_apb.topsw_lsbus_clk_ctrl.write(
                crf_apb::TopswLsbusClkCtrl::zeroed()
                    .clkact(true)
                    .divisor0(5)
                    .srcsel(2),
            );
        });
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
