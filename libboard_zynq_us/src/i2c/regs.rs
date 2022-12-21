//! I2C registers

use libregister::{register, register_at, register_bit, register_bits};

#[repr(C)]
pub struct RegisterBlock {
    pub control: Control,
    pub status: Status,
    pub addr: Addr,
    pub data: Data,
    pub isr: ISR,
    pub tx_size: TxSize,
    pub slv_mon_pause: SlvMonPause,
    pub timeout: Timeout,
    pub interrupt_mask: InterruptMask,
    pub interrupt_enable: InterruptEnable,
    pub interrupt_disable: InterruptDisable,
    pub glitch_filter: GlitchFilter,
}

register_at!(RegisterBlock, 0xFF02_0000, i2c0);
register_at!(RegisterBlock, 0xFF03_0000, i2c1);

register!(control, Control, RW, u32);
// div += 1
register_bits!(control, divisor_a, u8, 14, 15);
register_bits!(control, divisor_b, u8, 8, 13);
// self-clearing
register_bit!(control, clear_fifo, 6);
// 1: monitor mode, 0: normal
register_bit!(control, slv_mon, 5);
register_bit!(control, hold, 4);
register_bit!(control, ack_en, 3);
// 1: normal (7-bit), 0: extended (10-bit)
register_bit!(control, addr_mode, 2);
// 1: master, 0: slave
register_bit!(control, interface_mode, 1);
// (master mode) 1: RX, 0: TX
register_bit!(control, rx_en, 0);

register!(status, Status, RO, u32);
register_bit!(status, bus_active, 8);
register_bit!(status, rx_overflow, 7);
// UG 1087: SW should not use this to determine data completion, it is the RAW value on the interface.
// Please use COMP in the ISR.
register_bit!(status, tx_data_valid, 6);
register_bit!(status, rx_data_valid, 5);
register_bit!(status, rx_rw, 3);

register!(addr, Addr, RW, u32);
register_bits!(addr, addr, u16, 0, 9);

register!(data, Data, RW, u32);
register_bits!(data, data, u8, 0, 7);

register!(isr, ISR, RW, u32);
register_bit!(isr, arb_lost, 9, WTC);
register_bit!(isr, rx_underflow, 7, WTC);
register_bit!(isr, tx_overflow, 6, WTC);
register_bit!(isr, rx_overflow, 5, WTC);
register_bit!(isr, slv_ready, 4, WTC);
register_bit!(isr, timeout, 3, WTC);
register_bit!(isr, nack, 2, WTC);
register_bit!(isr, data, 1, WTC);
register_bit!(isr, tx_complete, 0, WTC);

register!(tx_size, TxSize, RW, u32);
register_bits!(tx_size, tx_size, u8, 0, 7);

register!(slv_mon_pause, SlvMonPause, RW, u32);
register_bits!(slv_mon_pause, pause, u8, 0, 3);

register!(timeout, Timeout, RW, u32);
register_bits!(timeout, timeout, u8, 0, 7);

register!(interrupt_mask, InterruptMask, RO, u32);
register_bit!(interrupt_mask, arb_lost, 9);
register_bit!(interrupt_mask, rx_underflow, 7);
register_bit!(interrupt_mask, tx_overflow, 6);
register_bit!(interrupt_mask, rx_overflow, 5);
register_bit!(interrupt_mask, slv_ready, 4);
register_bit!(interrupt_mask, timeout, 3);
register_bit!(interrupt_mask, nack, 2);
register_bit!(interrupt_mask, data, 1);
register_bit!(interrupt_mask, tx_complete, 0);

register!(interrupt_enable, InterruptEnable, RO, u32);
register_bit!(interrupt_enable, arb_lost, 9);
register_bit!(interrupt_enable, rx_underflow, 7);
register_bit!(interrupt_enable, tx_overflow, 6);
register_bit!(interrupt_enable, rx_overflow, 5);
register_bit!(interrupt_enable, slv_ready, 4);
register_bit!(interrupt_enable, timeout, 3);
register_bit!(interrupt_enable, nack, 2);
register_bit!(interrupt_enable, data, 1);
register_bit!(interrupt_enable, tx_complete, 0);

register!(interrupt_disable, InterruptDisable, RO, u32);
register_bit!(interrupt_disable, arb_lost, 9);
register_bit!(interrupt_disable, rx_underflow, 7);
register_bit!(interrupt_disable, tx_overflow, 6);
register_bit!(interrupt_disable, rx_overflow, 5);
register_bit!(interrupt_disable, slv_ready, 4);
register_bit!(interrupt_disable, timeout, 3);
register_bit!(interrupt_disable, nack, 2);
register_bit!(interrupt_disable, data, 1);
register_bit!(interrupt_disable, tx_complete, 0);

register!(glitch_filter, GlitchFilter, RW, u32);
register_bits!(glitch_filter, glitch_filter, u8, 0, 3);