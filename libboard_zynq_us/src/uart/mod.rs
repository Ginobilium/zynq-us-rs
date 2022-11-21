// Copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libboard_zynq/src/uart/baud_rate_gen.rs
// Original authors: Astro, Harry Ho, Sebastien Bourdeauducq
// Modifications made for different MIO pins and SLCRs

use libregister::{RegisterR, RegisterRW, RegisterW};

use super::clocks::Clocks;
use super::slcr::{common::Unlocked, crl_apb, iou_slcr};

mod baud_rate_gen;
mod regs;

pub struct Uart {
    regs: &'static mut regs::RegisterBlock,
}

impl Uart {
    #[cfg(feature = "target_zcu111")]
    pub fn uart0(baudrate: u32) -> Self {
        iou_slcr::RegisterBlock::unlocked(|slcr| {
            // UART0 RxD
            slcr.mio_pin[18].write(iou_slcr::MioPin::zeroed().l3_sel(0b110));
            slcr.mio_pull_enable(18, true);
            slcr.mio_pullup(18, true);
            slcr.mio_tri_enable(18, true);

            // UART0 TxD
            slcr.mio_pin[19].write(iou_slcr::MioPin::zeroed().l3_sel(0b110));
            slcr.mio_pull_enable(19, true);
            slcr.mio_pullup(19, true);
        });

        crl_apb::RegisterBlock::unlocked(|slcr| {
            // reset uart
            slcr.peri_rst_ctrl.modify(|_, w| w.uart0_rst(true));
            slcr.peri_rst_ctrl.modify(|_, w| w.uart0_rst(false));
            slcr.uart0_clk_ctrl.modify(|_, w| w.clkact(true));
        });

        let mut self_ = Uart {
            regs: regs::RegisterBlock::uart0(),
        };
        self_.configure(baudrate);
        self_
    }

    #[cfg(not(feature = "target_zcu111"))]
    pub fn uart1(baudrate: u32) -> Self {
        iou_slcr::RegisterBlock::unlocked(|slcr| {
            // UART1 RxD
            slcr.mio_pin[21].write(iou_slcr::MioPin::zeroed().l3_sel(0b110));
            slcr.mio_pull_enable(21, true);
            slcr.mio_pullup(21, true);
            slcr.mio_tri_enable(21, true);

            // UART1 TxD
            slcr.mio_pin[20].write(iou_slcr::MioPin::zeroed().l3_sel(0b110));
            slcr.mio_pull_enable(20, true);
            slcr.mio_pullup(20, true);
        });

        crl_apb::RegisterBlock::unlocked(|slcr| {
            // reset uart
            slcr.peri_rst_ctrl.modify(|_, w| w.uart1_rst(true));
            slcr.peri_rst_ctrl.modify(|_, w| w.uart1_rst(false));
            slcr.uart1_clk_ctrl.modify(|_, w| w.clkact(true));
        });

        let mut self_ = Uart {
            regs: regs::RegisterBlock::uart1(),
        };
        self_.configure(baudrate);
        self_
    }

    pub fn write_byte(&mut self, value: u8) {
        while self.tx_fifo_full() {}

        self.regs
            .tx_rx_fifo
            .write(regs::TxRxFifo::zeroed().data(value.into()));
    }

    pub fn configure(&mut self, baudrate: u32) {
        // Configure UART character frame
        // * Disable clock-divider
        // * 8-bit
        // * 1 stop bit
        // * Normal channel mode
        // * No parity
        self.regs.mode.write(
            regs::Mode::zeroed()
                .par(regs::ParityMode::None)
                .chmode(regs::ChannelMode::Normal),
        );

        // Configure the Baud Rate
        self.disable_rx();
        self.disable_tx();

        let clocks = Clocks::get();
        baud_rate_gen::configure(self.regs, clocks.uart0_ref_clk(), baudrate);

        // Enable controller
        self.reset_rx();
        self.reset_tx();
        self.wait_reset();
        self.enable_rx();
        self.enable_tx();

        self.set_rx_timeout(false);
        self.set_break(false, true);
    }

    fn disable_rx(&mut self) {
        self.regs.control.modify(|_, w| w.rxen(false).rxdis(true))
    }

    fn disable_tx(&mut self) {
        self.regs.control.modify(|_, w| w.txen(false).txdis(true))
    }

    fn enable_rx(&mut self) {
        self.regs.control.modify(|_, w| w.rxen(true).rxdis(false))
    }

    fn enable_tx(&mut self) {
        self.regs.control.modify(|_, w| w.txen(true).txdis(false))
    }

    fn reset_rx(&mut self) {
        self.regs.control.modify(|_, w| {
            w.rxrst(true) // self-clearing once reset is complete
        })
    }

    fn reset_tx(&mut self) {
        self.regs.control.modify(|_, w| {
            w.txrst(true) // self-clearing once reset is complete
        })
    }

    /// Wait for `reset_rx()` or `reset_tx()` to complete
    fn wait_reset(&self) {
        let mut pending = true;
        while pending {
            let control = self.regs.control.read();
            pending = control.rxrst() || control.txrst();
        }
    }

    fn set_break(&mut self, startbrk: bool, stopbrk: bool) {
        self.regs
            .control
            .modify(|_, w| w.sttbrk(startbrk).stpbrk(stopbrk))
    }

    // 0 disables
    fn set_rx_timeout(&mut self, enable: bool) {
        self.regs.control.modify(|_, w| w.rstto(enable))
    }

    pub fn tx_fifo_full(&self) -> bool {
        self.regs.channel_sts.read().txfull()
    }

    pub fn tx_idle(&self) -> bool {
        let status = self.regs.channel_sts.read();
        status.txempty() && !status.tactive()
    }
}
