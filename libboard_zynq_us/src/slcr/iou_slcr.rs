use libregister::{register, register_at, register_bit, register_bits, RegisterRW};
///! IOU SLCR for MIO pin configuration
use volatile_register::{RO, RW, WO};

use super::common::Unlocked;

pub const NUM_MIO_PINS: u8 = 78;
pub const NUM_BANKS: u8 = 3;

#[repr(C)]
pub struct RegisterBlock {
    pub mio_pin: [MioPin; NUM_MIO_PINS as usize],
    pub bank_csr: [BankCSR; NUM_BANKS as usize],
    unused1: [u32; 5],
    pub mio_loopback: RW<u32>,
    pub mio_tri_enable: [MioTriEnable; NUM_BANKS as usize],
    pub wdt_clk_sel: RW<u32>, // 0 = internal APB clock, 1 = external
    pub can_mio_ctrl: RW<u32>,
    pub gem_clk_ctrl: RW<u32>,
    pub sdio_clk_ctrl: RW<u32>,
    pub ctrl_reg_sd: RW<u32>,
    pub sd_itap_dly: RW<u32>,
    pub sd_otap_dly_sel: RW<u32>,
    pub sd_cfg1: RW<u32>,
    pub sd_cfg2: RW<u32>,
    pub sd_cfg3: RW<u32>,
    pub sd_init_preset: RW<u32>,
    pub sd_speed_preset: RW<u32>,
    pub sd_hspeed_preset: RW<u32>,
    pub sd_sdr12_preset: RW<u32>,
    pub sd_sdr25_preset: RW<u32>,
    pub sd_sdr50_preset: RW<u32>,
    unused2: [u32; 1],
    pub sd_sdr104_preset: RW<u32>,
    pub sd_ddr50_preset: RW<u32>,
    pub sd_max_cur_18: RW<u32>,
    pub sd_max_cur_30: RW<u32>,
    pub sd_max_cur_33: RW<u32>,
    pub sd_dll_ctrl: RW<u32>,
    pub sd_cdn_ctrl: RW<u32>,
    pub gem_ctrl: RW<u32>,
    unused3: [u32; 7],
    pub iou_ttc_apb_clk: RW<u32>,
    unused4: [u32; 3],
    pub iou_tapdly_bypass: RW<u32>,
    unused5: [u32; 3],
    pub iou_coherent_ctrl: RW<u32>,
    pub video_pss_clk_sel: RW<u32>,
    pub iou_interconnect_route: RW<u32>,
    unused6: [u32; 125],
    pub ctrl: RW<u32>,
    unused7: [u32; 63],
    pub isr: RW<u32>, // todo: WTC LSB
    pub imr: RO<u32>,
    pub ier: WO<u32>,
    pub idr: WO<u32>,
    pub itr: WO<u32>,
}
register_at!(RegisterBlock, 0xFF18_0000, slcr);

pub struct BankCSR {
    pub bank_drive_ctrl: [BankDriveCtrl; 2],
    pub bank_input_ctrl: BankInputCtrl,
    pub bank_pull_ctrl: BankPullCtrl,
    pub bank_pull_enable: BankPullEnable,
    pub bank_slew_ctrl: BankSlewCtrl,
    pub bank_status: BankStatus,
}

impl Unlocked for RegisterBlock {
    // Dummy definition for consistency
    fn unlocked<F: FnMut(&mut Self) -> R, R>(mut f: F) -> R {
        let mut self_ = Self::slcr();
        f(&mut self_)
    }
}

impl RegisterBlock {
    pub fn mio_pullup(&mut self, pin: u8, pullup: bool) {
        let bank = pin / NUM_BANKS;
        let idx = pin % bank;
        self.bank_csr[bank as usize]
            .bank_pull_ctrl
            .modify(|r, w| w.pullup(r.pullup() | (u32::from(pullup) << idx)));
    }

    pub fn mio_pull_enable(&mut self, pin: u8, enable: bool) {
        let bank = pin / NUM_BANKS;
        let idx = pin % bank;
        self.bank_csr[bank as usize]
            .bank_pull_enable
            .modify(|r, w| w.pull_enable(r.pull_enable() | (u32::from(enable) << idx)));
    }

    pub fn mio_tri_enable(&mut self, pin: u8, enable: bool) {
        // because why organize register fields in a consistent way
        let bank = pin / 32;
        let idx = pin % 32;
        self.mio_tri_enable[bank as usize].modify(|r, _| mio_tri_enable::Write {
            inner: r.inner | (u32::from(enable) << idx),
        });
    }
}

register!(mio_pin, MioPin, RW, u32);
register_bits!(mio_pin, l3_sel, u8, 5, 7);
register_bits!(mio_pin, l2_sel, u8, 3, 4);
register_bit!(mio_pin, l1_sel, 2);
register_bit!(mio_pin, l0_sel, 1);

register!(bank_drive_ctrl, BankDriveCtrl, RW, u32);
register_bits!(bank_drive_ctrl, drive, u32, 0, 25);

// 0 = CMOS, 1 = Schmitt
register!(bank_input_ctrl, BankInputCtrl, RW, u32);
register_bits!(bank_input_ctrl, schmitt, u32, 0, 25);

// 0 = down, 1 = up
register!(bank_pull_ctrl, BankPullCtrl, RW, u32);
register_bits!(bank_pull_ctrl, pullup, u32, 0, 25);

register!(bank_pull_enable, BankPullEnable, RW, u32);
register_bits!(bank_pull_enable, pull_enable, u32, 0, 25);

// 0 = fast, 1 = slow
register!(bank_slew_ctrl, BankSlewCtrl, RW, u32);
register_bits!(bank_slew_ctrl, slow_slew, u32, 0, 25);

// 0 = 2.5 or 3.3V, 1 = 1.8V
register!(bank_status, BankStatus, RO, u32);
register_bit!(bank_status, voltage_mode, 0);

register!(mio_tri_enable, MioTriEnable, RW, u32);
