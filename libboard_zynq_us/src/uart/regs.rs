// Copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libboard_zynq/src/uart/regs.rs
// Original authors: Astro
// Modifications made for additional register and different block addresses.

use volatile_register::{RW};

use libregister::{register, register_at, register_bit, register_bits, register_bits_typed};

#[allow(unused)]
#[repr(u8)]
pub enum ChannelMode {
    Normal = 0b00,
    AutomaticEcho = 0b01,
    LocalLoopback = 0b10,
    RemoteLoopback = 0b11,
}

#[allow(unused)]
#[repr(u8)]
pub enum ParityMode {
    EvenParity = 0b000,
    OddParity = 0b001,
    ForceTo0 = 0b010,
    ForceTo1 = 0b011,
    None = 0b100,
}

#[allow(unused)]
#[repr(u8)]
pub enum StopBits {
    One = 0b00,
    OneAndHalf = 0b01,
    Two = 0b10,
}

#[allow(unused)]
#[repr(u8)]
pub enum CharacterLength {
    Eight = 0b00, // actually 0b0x
    Seven = 0b10,
    Six = 0b11,
}

#[repr(C)]
pub struct RegisterBlock {
    pub control: Control,
    pub mode: Mode,
    pub interrupt_enable: InterruptEnable,
    pub interrupt_disable: InterruptDisable,
    pub interrupt_mask: InterruptMask,
    pub channel_interrupt_status: ChannelInterruptStatus,
    pub baud_rate_gen: BaudRateGen,
    pub rcvr_timeout: RcvrTimeout,
    pub rcvr_fifo_trigger_level: FifoTriggerLevel,
    pub modem_ctrl: RW<u32>,
    pub modem_sts: ModemSts,
    pub channel_sts: ChannelSts,
    pub tx_rx_fifo: TxRxFifo,
    pub baud_rate_divider: BaudRateDiv,
    pub flow_delay: RW<u32>,
    unused0: [u32; 2],
    pub tx_fifo_trigger_level: FifoTriggerLevel,
    pub rx_fifo_byte_status: RW<u32>,
}
register_at!(RegisterBlock, 0xFF000000, uart0);
register_at!(RegisterBlock, 0xFF010000, uart1);

register!(control, Control, RW, u32);
register_bit!(control, rxrst, 0);
register_bit!(control, txrst, 1);
register_bit!(control, rxen, 2);
register_bit!(control, rxdis, 3);
register_bit!(control, txen, 4);
register_bit!(control, txdis, 5);
register_bit!(control, rstto, 6);
register_bit!(control, sttbrk, 7);
register_bit!(control, stpbrk, 8);

register!(mode, Mode, RW, u32);
// Configure the size of FIFO access from the APB
register_bits!(mode, wsize, u8, 12, 13);
// Channel mode: Defines the mode of operation of the UART.
register_bits_typed!(mode, chmode, u8, ChannelMode, 8, 9);
// Number of stop bits
register_bits_typed!(mode, nbstop, u8, StopBits, 6, 7);
// Parity type select
register_bits_typed!(mode, par, u8, ParityMode, 3, 5);
// Character length select
register_bits_typed!(mode, chrl, u8, CharacterLength, 1, 2);
// Clock source select (0 = UART_REF_CLK, 1 = UART_REF_CLK / 8)
register_bit!(mode, clks, 0);

macro_rules! register_interrupt_bits {
    ($mod_name: ident) => {
        register_bit!($mod_name, rx_brk, 13);
        register_bit!($mod_name, tx_overflow, 12);
        register_bit!($mod_name, tx_nfull, 11);
        register_bit!($mod_name, tx_trig, 10);
        register_bit!($mod_name, dmsi, 9);
        register_bit!($mod_name, rx_timeout, 8);
        register_bit!($mod_name, rx_par, 7);
        register_bit!($mod_name, rx_frame, 6);
        register_bit!($mod_name, rx_overflow, 5);
        register_bit!($mod_name, tx_full, 4);
        register_bit!($mod_name, tx_empty, 3);
        register_bit!($mod_name, rx_full, 2);
        register_bit!($mod_name, rx_empty, 1);
        register_bit!($mod_name, rx_trig, 0);
    };

    ($mod_name: ident, WTC) => {
        register_bit!($mod_name, rx_brk, 13, WTC);
        register_bit!($mod_name, tx_overflow, 12, WTC);
        register_bit!($mod_name, tx_nfull, 11, WTC);
        register_bit!($mod_name, tx_trig, 10, WTC);
        register_bit!($mod_name, dmsi, 9, WTC);
        register_bit!($mod_name, rx_timeout, 8, WTC);
        register_bit!($mod_name, rx_par, 7, WTC);
        register_bit!($mod_name, rx_frame, 6, WTC);
        register_bit!($mod_name, rx_overflow, 5, WTC);
        register_bit!($mod_name, tx_full, 4, WTC);
        register_bit!($mod_name, tx_empty, 3, WTC);
        register_bit!($mod_name, rx_full, 2, WTC);
        register_bit!($mod_name, rx_empty, 1, WTC);
        register_bit!($mod_name, rx_trig, 0, WTC);
    };
}

register!(interrupt_enable, InterruptEnable, WO, u32);
register_interrupt_bits!(interrupt_enable);

register!(interrupt_disable, InterruptDisable, WO, u32);
register_interrupt_bits!(interrupt_disable);

register!(interrupt_mask, InterruptMask, RO, u32);
register_interrupt_bits!(interrupt_mask);

register!(channel_interrupt_status, ChannelInterruptStatus, RW, u32);
register_interrupt_bits!(channel_interrupt_status, WTC);

register!(baud_rate_gen, BaudRateGen, RW, u32);
register_bits!(baud_rate_gen, cd, u16, 0, 15);

register!(rcvr_timeout, RcvrTimeout, RW, u32);
register_bits!(rcvr_timeout, rto, u8, 0, 7);

register!(fifo_trigger_level, FifoTriggerLevel, RW, u32);
register_bits!(fifo_trigger_level, level, u8, 0, 5);

register!(modem_sts, ModemSts, RW, u32);
register_bit!(modem_sts, fcms, 8);
register_bit!(modem_sts, dcd, 7, RO);
register_bit!(modem_sts, ri, 6, RO);
register_bit!(modem_sts, dsr, 5, RO);
register_bit!(modem_sts, cts, 4, RO);
register_bit!(modem_sts, ddcd, 3, WTC);
register_bit!(modem_sts, teri, 2, WTC);
register_bit!(modem_sts, ddsr, 1, WTC);
register_bit!(modem_sts, dcts, 0, WTC);

register!(channel_sts, ChannelSts, RO, u32);
// Transmitter FIFO Nearly Full
register_bit!(channel_sts, tnful, 14);
// Tx FIFO fill level is greater than or equal to TTRIG?
register_bit!(channel_sts, ttrig, 13);
// Rx FIFO fill level is greater than or equal to FDEL?
register_bit!(channel_sts, flowdel, 12);
// Transmitter state machine active?
register_bit!(channel_sts, tactive, 11);
// Receiver state machine active?
register_bit!(channel_sts, ractive, 10);
// Tx FIFO is full?
register_bit!(channel_sts, txfull, 4);
// Tx FIFO is empty?
register_bit!(channel_sts, txempty, 3);
// Rx FIFO is full?
register_bit!(channel_sts, rxfull, 2);
// Rx FIFO is empty?
register_bit!(channel_sts, rxempty, 1);
// Rx FIFO fill level is greater than or equal to RTRIG?
register_bit!(channel_sts, rxovr, 0);

register!(tx_rx_fifo, TxRxFifo, RW, u32);
register_bits!(tx_rx_fifo, data, u8, 0, 7);

register!(baud_rate_div, BaudRateDiv, RW, u32);
register_bits!(baud_rate_div, bdiv, u8, 0, 7);
