//! I2C Driver and Registers
use libregister::{RegisterR, RegisterRW, RegisterW};
use log::{debug, error};

use super::clocks::Clocks;
use super::slcr::{common::Unlocked, crl_apb};
use super::util::div_round_closest;

pub mod regs;

// Hardware FIFO size in bytes
// u16 to avoid excessive casting
const FIFO_SIZE: u16 = 16;
// Threshold at which the `data` interrupt is triggered(?)
#[allow(unused)]
const DATA_INTR_THRESH: u8 = 14;
// Maximum size of master transfers in bytes
// UG1085 says this is 255 but the FSBL says otherwise
const MAX_TX: u8 = 252;
// Mux address (same for both)
#[cfg(feature = "target_zcu111")]
pub const MUX_ADDR: u16 = 0x75;
// Max SCL freq (DS926 Table 47)
const MAX_SCLK_FREQ: u32 = 400_000;
// Max divisors (note: actual divisor is register value + 1)
const MAX_DIV_A: u32 = 4;
const MAX_DIV_B: u32 = 64;

pub struct I2C {
    regs: &'static mut regs::RegisterBlock,
    ref_clk: u32,
}

impl I2C {
    /// Constructor for I2C0 controller.
    pub fn i2c0() -> Self {
        crl_apb::RegisterBlock::unlocked(|crl_apb| {
            crl_apb.peri_rst_ctrl.modify(|_, w| w.i2c0_rst(true));
            crl_apb.peri_rst_ctrl.modify(|_, w| w.i2c0_rst(false));
        });
        let mut self_ = I2C {
            regs: regs::RegisterBlock::i2c0(),
            ref_clk: Clocks::get().i2c0_ref_clk(),
        };
        self_.reset(false);
        self_
    }

    /// Constructor for I2C1 controller.
    pub fn i2c1() -> Self {
        crl_apb::RegisterBlock::unlocked(|crl_apb| {
            crl_apb.peri_rst_ctrl.modify(|_, w| w.i2c1_rst(true));
            crl_apb.peri_rst_ctrl.modify(|_, w| w.i2c1_rst(false));
        });
        let mut self_ = I2C {
            regs: regs::RegisterBlock::i2c1(),
            ref_clk: Clocks::get().i2c1_ref_clk(),
        };
        self_.reset(false);
        self_
    }

    /// Reset the I2C controller.
    ///
    /// UG1085 Table 22-3: I2C Reset
    ///
    /// * `restore_interrupts`: `bool` - whether to restore previously-enabled interrupts after reset
    pub fn reset(&mut self, restore_interrupts: bool) {
        // save current interrupt config before disabling interrupts
        let imr: u32 = self.regs.interrupt_mask.read().inner;
        self.disable_interrupts();

        // clear FIFO
        self.regs
            .control
            .write(regs::Control::zeroed().clear_fifo(true));

        // clear interrupts
        self.clear_interrupt_status();

        // restore enabled interrupts
        if restore_interrupts {
            self.regs
                .interrupt_enable
                .write(regs::interrupt_enable::Write { inner: imr });
        }
    }

    pub fn set_sclk(&mut self, mut freq: u32) {
        assert!(
            freq <= MAX_SCLK_FREQ,
            "SCL frequency cannot exceed {} Hz",
            MAX_SCLK_FREQ
        );
        let target_div = div_round_closest(self.ref_clk, freq);
        assert!(
            target_div <= (MAX_DIV_A * MAX_DIV_B),
            "Target frequency not achievable with current reference clock"
        );
        // from the FSBL (and I dare not tempt fate):
        // > If frequency 400KHz is selected, 384.6KHz should be set.
        // > If frequency 100KHz is selected, 90KHz should be set.
        // > This is due to a hardware limitation.
        if freq > 384_600 {
            freq = 384_600;
        } else if freq > 90_000 && freq <= 100_000 {
            freq = 90_000;
        }

        let mut best_error_hz = freq;
        let mut best_div_a: u32 = 0;
        let mut best_div_b: u32 = 0;

        for div_a in 1..=MAX_DIV_A {
            let div_b = div_round_closest(target_div, div_a).min(MAX_DIV_B);
            let error_hz = div_round_closest(self.ref_clk, div_a * div_b).abs_diff(freq);
            if error_hz < best_error_hz {
                best_div_a = div_a;
                best_div_b = div_b;
                best_error_hz = error_hz;
            }
        }

        // shouldn't happen
        debug_assert_ne!(best_div_a, 0);
        debug_assert_ne!(best_div_b, 0);

        debug!("Divisors: {best_div_a}, {best_div_b}, error: {best_error_hz} Hz for target frequency {freq} Hz");

        self.regs.control.modify(|_, w| {
            w.divisor_a((best_div_a - 1) as u8)
                .divisor_b((best_div_b - 1) as u8)
        });
        // TODO: set glitch filter?
    }

    /// Perform common setup steps for master mode.
    ///
    /// Sets the interface to master mode, clears the FIFO & interrupt status, sets RX/TX according
    /// to the `rx_en` argument, and disables all interrupts.
    pub fn master_setup(&mut self, rx_en: bool) {
        self.set_interface_mode(true);
        self.set_ack_en(true);
        self.clear_fifo();
        self.clear_interrupt_status();
        self.set_rx_en(rx_en);
        self.disable_interrupts();
    }

    /// Perform a polled write in master mode.
    ///
    /// UG1085 Chapter 22 - Programming Model and Table 22-12
    ///
    /// * `addr`: `u16` - address to read from (7 or 10 bits depending on addressing mode)
    /// * `size`: `u16` - transfer size in bytes
    /// * `data`: `&[u8]` - data to send
    ///
    /// Returns:
    /// * `Ok(())` if transfer was successful
    /// * `Err(u32)` containing the last read value of [crate::i2c::regs::interrupt_status] if the transfer failed
    pub fn master_write_polled(&mut self, addr: u16, size: u16, data: &[u8]) -> Result<(), u32> {
        let n_fill = size / FIFO_SIZE;
        let rem = size % FIFO_SIZE;
        self.regs.control.modify(|_, w| w.hold(size > FIFO_SIZE));

        self.master_setup(false);

        self.regs.addr.modify(|_, w| w.addr(addr));

        let mut isr_read: regs::interrupt_status::Read;
        // send the first n_fill * FIFO_SIZE chunks
        for i in 0..n_fill {
            // fill fifo
            for j in 0..FIFO_SIZE {
                self.regs
                    .data
                    .modify(|_, w| w.data(data[(i * FIFO_SIZE + j) as usize]));
            }

            // wait for empty
            loop {
                isr_read = self.regs.interrupt_status.read();
                if Self::tx_error(&isr_read) || !self.regs.status.read().tx_data_valid() {
                    break;
                }
            }

            if Self::tx_error(&isr_read) {
                // TODO: self.reset()?
                error!("Failed to complete transaction. Last read value of ISR: {:#X}", isr_read.inner);
                return Err(isr_read.inner);
            }
        }

        // TODO: set hold to false?
        // send remainder (or the entire slice if smaller than FIFO_SIZE)
        for i in 0..rem {
            self.regs
                .data
                .modify(|_, w| w.data(data[(n_fill * FIFO_SIZE + i) as usize]));
        }

        // wait for tx completion
        loop {
            isr_read = self.regs.interrupt_status.read();
            if Self::tx_error(&isr_read) || isr_read.tx_complete() {
                break;
            }
        }

        if Self::tx_error(&isr_read) {
            error!("Failed to complete transaction. Last read value of ISR: {:#X}", isr_read.inner);
            return Err(isr_read.inner);
        }

        Ok(())
    }

    /// Perform a polled read in master mode.
    ///
    /// UG1085 Chapter 22 - Programming Model and Table 22-13
    ///
    /// * `addr`: `u16` - address to read from (7 or 10 bits depending on addressing mode)
    /// * `size`: `u16` - transfer size in bytes
    /// * `rx_buffer`: `&mut [u8]` - mutable slice to put the read data in
    ///
    /// Returns:
    /// * `Ok(())` if transfer was successful
    /// * `Err(u32)` containing the last read value of [crate::i2c::regs::interrupt_status] if the transfer failed
    pub fn master_read_polled(
        &mut self,
        addr: u16, // TODO: make option and default to buffer size
        size: u16,
        rx_buffer: &mut [u8],
    ) -> Result<(), u32> {
        assert!(
            size as usize <= rx_buffer.len(),
            "Buffer (len {} not big enough for transfer size ({})",
            rx_buffer.len(),
            size
        );
        self.master_setup(true);

        let mut hold = size > FIFO_SIZE;
        self.regs.control.modify(|_, w| w.hold(hold));

        self.regs.addr.write(regs::Addr::zeroed().addr(addr));

        // total (not per-chunk) bytes remaining
        let mut remaining_bytes = size;
        let mut isr_read: regs::interrupt_status::Read;
        while remaining_bytes > 0 {
            debug!("Remaining bytes: {}", remaining_bytes);
            if remaining_bytes > MAX_TX as u16 {
                self.regs
                    .tx_size
                    .write(regs::TxSize::zeroed().tx_size(MAX_TX));
            } else {
                self.regs
                    .tx_size
                    .write(regs::TxSize::zeroed().tx_size(remaining_bytes as u8));
            }
            // TODO: rewrite addr for each chunk?
            isr_read = self.regs.interrupt_status.read();
            while self.regs.status.read().rx_data_valid() && !Self::rx_error(&isr_read) {
                rx_buffer[(size - remaining_bytes) as usize] = self.regs.data.read().data();
                remaining_bytes -= 1;
                if hold && remaining_bytes < FIFO_SIZE {
                    hold = false;
                    self.regs.control.modify(|_, w| w.hold(false));
                }
            }

            if Self::rx_error(&isr_read) {
                // TODO: self.reset()?
                error!("Failed to complete transaction. Last read value of ISR: {:#X}", isr_read.inner);
                return Err(isr_read.inner);
            }
        }

        Ok(())
    }

    /// Clear the FIFO and TX size register.
    pub fn clear_fifo(&mut self) {
        // self-clearing bit
        self.regs.control.modify(|_, w| w.clear_fifo(true));
    }

    /// Set ACK enable.
    pub fn set_ack_en(&mut self, enable: bool) {
        self.regs.control.modify(|_, w| w.ack_en(enable));
    }

    /// (Master mode) Set addressing mode.
    ///
    /// * `mode`: `bool` - `true` for 7-bit (normal), `false` for 10-bit (extended)
    pub fn set_addr_mode(&mut self, mode: bool) {
        self.regs.control.modify(|_, w| w.addr_mode(mode));
    }

    /// Set interface mode.
    ///
    /// * `mode`: `bool` - `true` for master, `false` for slave mode
    pub fn set_interface_mode(&mut self, mode: bool) {
        self.regs.control.modify(|_, w| w.interface_mode(mode));
    }

    /// (Master mode) RX/TX selection.
    ///
    /// * `enable`: `bool` - `true` for RX, `false` for TX
    pub fn set_rx_en(&mut self, enable: bool) {
        self.regs.control.modify(|_, w| w.rx_en(enable));
    }

    /// Clear flags in ISR.
    pub fn clear_interrupt_status(&mut self) {
        self.regs.interrupt_status.modify(|r, _| {
            // clear currently set flags
            regs::interrupt_status::Write { inner: r.inner }
        });
    }

    /// Enable all interrupts
    pub fn enable_interrupts(&mut self) {
        self.regs.interrupt_enable.write(
            regs::InterruptEnable::zeroed()
                .arb_lost(true)
                .rx_underflow(true)
                .tx_overflow(true)
                .rx_overflow(true)
                .slv_ready(true)
                .timeout(true)
                .nack(true)
                .data(true)
                .tx_complete(true),
        );
    }

    /// Disable all interrupts
    pub fn disable_interrupts(&mut self) {
        self.regs.interrupt_disable.write(
            regs::InterruptDisable::zeroed()
                .arb_lost(true)
                .rx_underflow(true)
                .tx_overflow(true)
                .rx_overflow(true)
                .slv_ready(true)
                .timeout(true)
                .nack(true)
                .data(true)
                .tx_complete(true),
        );
    }

    /// Returns whether any RX-related error flags are set
    #[inline]
    fn rx_error(isr_read: &regs::interrupt_status::Read) -> bool {
        isr_read.arb_lost() || isr_read.rx_underflow() || isr_read.rx_overflow() || isr_read.nack()
    }

    /// Returns whether any TX-related error flags are set
    #[inline]
    fn tx_error(isr_read: &regs::interrupt_status::Read) -> bool {
        isr_read.arb_lost() || isr_read.tx_overflow() || isr_read.nack()
    }

    /// Returns whether the I2C bus is busy.
    pub fn busy(&mut self) -> bool {
        self.regs.status.read().bus_active()
    }
}

