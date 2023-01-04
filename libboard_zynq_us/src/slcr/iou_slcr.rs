///! IOU SLCR for MIO pin configuration
use libregister::{register, register_at, register_bit, register_bits, RegisterW};
use volatile_register::{RO, RW, WO};

use super::common::Unlocked;

pub const NUM_MIO_PINS: usize = 78;
pub const NUM_BANKS: usize = 3;

#[repr(C)]
pub struct RegisterBlock {
    pub mio_pin: [MioPin; NUM_MIO_PINS],
    pub bank_csr: [BankCSR; NUM_BANKS],
    unused1: [u32; 29],
    pub mio_loopback: RW<u32>,
    pub mio_tri_enable: [MioTriEnable; NUM_BANKS],
    pub wdt_clk_sel: RW<u32>, // 0 = internal APB clock, 1 = external
    pub can_mio_ctrl: RW<u32>,
    pub gem_clk_ctrl: RW<u32>,
    pub sdio_clk_ctrl: SdioClkCtrl,
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
    /// Set up muxes, pull-ups, etc. according to target pin assignments
    pub fn mio_init(&mut self) {
        for pin in 0..NUM_MIO_PINS {
            let mio_sel = MIO_SEL[pin];
            // in retrospect it would have made a lot more sense to reverse the tuple order
            self.mio_pin[pin].write(
                MioPin::zeroed()
                    .l3_sel(mio_sel.0)
                    .l2_sel(mio_sel.1)
                    .l1_sel(mio_sel.2)
                    .l0_sel(mio_sel.3),
            );
        }
        for bank in 0..NUM_BANKS {
            let bank_csr = &mut self.bank_csr[bank];
            for i in 0..2 {
                bank_csr.bank_drive_ctrl[i]
                    .write(BankDriveCtrl::zeroed().drive(BANK_DRIVE_CTRL[bank][i]));
            }
            bank_csr
                .bank_input_ctrl
                .write(BankInputCtrl::zeroed().schmitt(BANK_INPUT_CTRL[bank]));
            bank_csr
                .bank_pull_ctrl
                .write(BankPullCtrl::zeroed().pullup(BANK_PULL_CTRL[bank]));
            bank_csr
                .bank_pull_enable
                .write(BankPullEnable::zeroed().pull_enable(BANK_PULL_ENABLE[bank]));
            bank_csr
                .bank_slew_ctrl
                .write(BankSlewCtrl::zeroed().slow_slew(BANK_SLEW_CTRL[bank]));

            self.mio_tri_enable[bank].write(MioTriEnable::zeroed().enable(MIO_TRI_ENABLE[bank]));
        }
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
register_bits!(mio_tri_enable, enable, u32, 0, 31);

register!(sdio_clk_ctrl, SdioClkCtrl, RW, u32);
// 0: MIO, 1: EMIO
register_bit!(sdio_clk_ctrl, sdio1_fb_clk_sel, 18);
// 0: MIO 51, 1: MIO 76
register_bit!(sdio_clk_ctrl, sdio1_rx_src_sel, 17);
// 0: MIO, 1: EMIO
register_bit!(sdio_clk_ctrl, sdio0_fb_clk_sel, 2);
// 00: MIO 22, 01: MIO 38, 1x: MIO 64
register_bits!(sdio_clk_ctrl, sdio0_rx_src_sel, u8, 0, 1);

// Target-specific constants
// (L3_SEL, L2_SEL, L1_SEL, L0_SEL)
#[cfg(feature = "target_zcu111")]
const MIO_SEL: [(u8, u8, bool, bool); NUM_MIO_PINS] = [
    (0, 0, false, true), // 0: QSPI
    (0, 0, false, true), // 1: QSPI
    (0, 0, false, true), // 2: QSPI
    (0, 0, false, true), // 3: QSPI
    (0, 0, false, true), // 4: QSPI
    (0, 0, false, true), // 5: QSPI
    // TODO: UG1271 says 6 is NC
    (0, 0, false, true),  // 6: QSPI
    (0, 0, false, true),  // 7: QSPI
    (0, 0, false, true),  // 8: QSPI
    (0, 0, false, true),  // 9: QSPI
    (0, 0, false, true),  // 10: QSPI
    (0, 0, false, true),  // 11: QSPI
    (0, 0, false, true),  // 12: QSPI
    (0, 0, false, false), // 13: GPIO
    (2, 0, false, false), // 14: I2C0 SCL
    (2, 0, false, false), // 15: I2C0 SDA
    (2, 0, false, false), // 16: I2C1 SCL
    (2, 0, false, false), // 17: I2C1 SDA
    (6, 0, false, false), // 18: UART0 RXD
    (6, 0, false, false), // 19: UART0 TXD
    (0, 0, false, false), // 20: NC
    (0, 0, false, false), // 21: NC
    (0, 0, false, false), // 22: GPIO
    (0, 0, false, false), // 23: GPIO
    (0, 0, false, false), // 24: NC
    (0, 0, false, false), // 25: NC
    // TODO: FSBL has 26 as all zeros but pmu in is l2_sel = 1
    (0, 0, false, false), // 26: PMU IN
    (0, 3, false, false), // 27: DP AUX
    (0, 3, false, false), // 28: DP AUX
    (0, 3, false, false), // 29: DP AUX
    (0, 3, false, false), // 30: DP AUX
    (0, 0, false, false), // 31: NC
    (0, 1, false, false), // 32: PMU OUT
    (0, 1, false, false), // 33: PMU OUT
    // TODO: pins 34-37 are just skipped in the FSBL
    (0, 1, false, false), // 34: PMU OUT
    (0, 1, false, false), // 35: PMU OUT
    (0, 1, false, false), // 36: PMU OUT
    (0, 1, false, false), // 37: PMU OUT
    (0, 0, false, false), // 38: GPIO
    (0, 2, false, false), // 39: SD1 data in/out[4]
    (0, 2, false, false), // 40: SD1 data in/out[5]
    (0, 2, false, false), // 41: SD1 data in/out[6]
    (0, 2, false, false), // 42: SD1 data in/out[7]
    (0, 0, false, false), // 43: NC
    (0, 0, false, false), // 44: NC
    (0, 2, false, false), // 45: SD1 CDn
    (0, 2, false, false), // 46: SD1 data in/out[0]
    (0, 2, false, false), // 47: SD1 data in/out[1]
    (0, 2, false, false), // 48: SD1 data in/out[2]
    (0, 2, false, false), // 49: SD1 data in/out[3]
    (0, 2, false, false), // 50: SD1 cmd in/out
    (0, 2, false, false), // 51: SD1 clk out
    (0, 0, true, false),  // 52: USB0 clk in
    (0, 0, true, false),  // 53: USB0 dir
    (0, 0, true, false),  // 54: USB0 data[2]
    (0, 0, true, false),  // 55: USB0 nxt
    (0, 0, true, false),  // 56: USB0 data[0]
    (0, 0, true, false),  // 57: USB0 data[1]
    (0, 0, true, false),  // 58: USB0 stop
    (0, 0, true, false),  // 59: USB0 data[3]
    (0, 0, true, false),  // 60: USB0 data[4]
    (0, 0, true, false),  // 61: USB0 data[5]
    (0, 0, true, false),  // 62: USB0 data[6]
    (0, 0, true, false),  // 63: USB0 data[7]
    (0, 0, false, true),  // 64: GEM3 tx clk
    (0, 0, false, true),  // 65: GEM3 txd[0]
    (0, 0, false, true),  // 66: GEM3 txd[1]
    (0, 0, false, true),  // 67: GEM3 txd[2]
    (0, 0, false, true),  // 68: GEM3 txd[3]
    (0, 0, false, true),  // 69: GEM3 tx ctl
    (0, 0, false, true),  // 70: GEM3 rx clk
    (0, 0, false, true),  // 71: GEM3 rxd[0]
    (0, 0, false, true),  // 72: GEM3 rxd[1]
    (0, 0, false, true),  // 73: GEM3 rxd[2]
    (0, 0, false, true),  // 74: GEM3 rxd[3]
    (0, 0, false, true),  // 75: GEM3 rx ctl
    (6, 0, false, false), // 76: MDIO3 clk
    (6, 0, false, false), // 77: MDIO3 data
];

#[cfg(feature = "target_zcu111")]
const MIO_TRI_ENABLE: [u32; NUM_BANKS] = [
    // Tri-state enable for pins 0-31
    (1 << 18) | (1 << 28) | (1 << 30),
    // Tri-state enable for pins 32-63
    (1 << (45 - 32)) | (1 << (52 - 32)) | (1 << (53 - 32)) | (1 << (55 - 32)),
    // Tri-state enable for pins 64-77
    (1 << (70 - 64))
        | (1 << (71 - 64))
        | (1 << (72 - 64))
        | (1 << (73 - 64))
        | (1 << (74 - 64))
        | (1 << (75 - 64)),
];

#[cfg(feature = "target_zcu111")]
const BANK_DRIVE_CTRL: [[u32; 2]; NUM_BANKS] = [
    [0x03FFFFFF, 0x03FFFFFF],
    [0x03FFFFFF, 0x03FFFFFF],
    [0x03FFFFFF, 0x03FFFFFF],
];

#[cfg(feature = "target_zcu111")]
const BANK_INPUT_CTRL: [u32; NUM_BANKS] = [0, 0, 0];

#[cfg(feature = "target_zcu111")]
const BANK_PULL_CTRL: [u32; NUM_BANKS] = [0x03FFFFFF, 0x03FFFFFF, 0x03FFFFFF];

#[cfg(feature = "target_zcu111")]
const BANK_PULL_ENABLE: [u32; NUM_BANKS] = [0x03FFFFFF, 0x03FFFFFF, 0x03FFFFFF];

#[cfg(feature = "target_zcu111")]
const BANK_SLEW_CTRL: [u32; NUM_BANKS] = [0, 0, 0];
