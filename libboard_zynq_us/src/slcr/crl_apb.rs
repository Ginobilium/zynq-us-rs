use libregister::{
    register, register_at, register_bit, register_bits, register_bits_typed, RegisterRW, RegisterW,
};
///! LPD clock and reset control
use volatile_register::{RO, RW, WO};

use super::common::{PllCfg, PllCtrl, PllFracCfg, Unlocked, WProt};

/// Clock source selection for IO-type devices
#[repr(u8)]
pub enum IoClkSource {
    IoPll = 0b00,
    RpuPll = 0b10,
    DdrPllToLpd = 0b11,
}

/// Clock source selection for RPU and related (e.g. LPD interconnect) devices
#[repr(u8)]
pub enum RpuClkSource {
    RpuPll = 0b00,
    IoPll = 0b10,
    DdrPllToLpd = 0b11,
}

#[repr(C)]
pub struct RegisterBlock {
    pub err_ctrl: RW<u32>,
    pub ir_status: RW<u32>, // todo: WTC LSB
    pub ir_mask: RO<u32>,
    pub ir_enable: WO<u32>,
    pub ir_disable: WO<u32>,
    unused1: [u32; 2],
    pub crl_wprot: WProt,
    pub io_pll_ctrl: PllCtrl,
    pub io_pll_cfg: PllCfg,
    pub io_pll_frac_cfg: PllFracCfg,
    unused2: [u32; 1],
    pub rpu_pll_ctrl: PllCtrl,
    pub rpu_pll_cfg: PllCfg,
    pub rpu_pll_frac_cfg: PllFracCfg,
    unused3: [u32; 1],
    pub pll_status: PllStatus,
    pub io_pll_to_fpd_ctrl: PllToFpdCtrl,
    pub rpu_pll_to_fpd_ctrl: PllToFpdCtrl,
    pub usb3_clk_ctrl: UsbClkCtrl,
    pub gem0_clk_ctrl: GemClkCtrl,
    pub gem1_clk_ctrl: GemClkCtrl,
    pub gem2_clk_ctrl: GemClkCtrl,
    pub gem3_clk_ctrl: GemClkCtrl,
    pub usb0_bus_clk_ctrl: UsbClkCtrl,
    pub usb1_bus_clk_ctrl: UsbClkCtrl,
    pub qspi_clk_ctrl: QSpiClkCtrl,
    pub sdio0_clk_ctrl: SdioClkCtrl,
    pub sdio1_clk_ctrl: SdioClkCtrl,
    pub uart0_clk_ctrl: UartClkCtrl,
    pub uart1_clk_ctrl: UartClkCtrl,
    pub spi0_clk_ctrl: SpiClkCtrl,
    pub spi1_clk_ctrl: SpiClkCtrl,
    pub can0_clk_ctrl: CanClkCtrl,
    pub can1_clk_ctrl: CanClkCtrl,
    unused4: [u32; 1],
    pub rpu_clk_ctrl: RpuClkCtrl,
    unused5: [u32; 2],
    pub iou_switch_clk_ctrl: IouSwitchClkCtrl,
    pub csu_clk_ctrl: CsuPllCtrl,
    pub pcap_clk_ctrl: PcapClkCtrl,
    pub lpd_switch_clk_ctrl: LpdSwitchClkCtrl,
    pub lpd_lsbus_clk_ctrl: LpdLsbusClkCtrl,
    pub dbg_lpd_clk_ctrl: DbgLpdClkCtrl,
    pub nand_clk_ctrl: NandClkCtrl,
    pub lpd_dma_clk_ctrl: LpdDmaClkCtrl,
    unused6: [u32; 1],
    pub pl0_clk_ctrl: PlClkCtrl,
    pub pl1_clk_ctrl: PlClkCtrl,
    pub pl2_clk_ctrl: PlClkCtrl,
    pub pl3_clk_ctrl: PlClkCtrl,
    pub pl0_thr_ctrl: PlThrCtrl,
    pub pl0_thr_cnt: PlThrCnt,
    pub pl1_thr_ctrl: PlThrCtrl,
    pub pl1_thr_cnt: PlThrCnt,
    pub pl2_thr_ctrl: PlThrCtrl,
    pub pl2_thr_cnt: PlThrCnt,
    pub pl3_thr_ctrl: PlThrCtrl,
    unused7: [u32; 4],
    pub pl3_thr_cnt: PlThrCnt,
    pub gem_tsu_clk_ctrl: GemTsuClkCtrl,
    pub dll_clk_ctrl: DllClkCtrl,
    pub ps_sysmon_clk_ctrl: PsSysmonClkCtrl,
    unused8: [u32; 5],
    pub i2c0_clk_ctrl: I2cClkCtrl,
    pub i2c1_clk_ctrl: I2cClkCtrl,
    pub timestamp_clk_ctrl: TimestampClkCtrl,
    unused9: [u32; 1],
    pub safety_chk: RW<u32>,
    unused10: [u32; 3],
    pub clkmon_status: RW<u32>,
    pub clkmon_mask: RO<u32>,
    pub clkmon_enable: WO<u32>,
    pub clkmon_disable: WO<u32>,
    pub clkmon_trigger: WO<u32>,
    unused11: [u32; 3],
    pub chkr0_clka_upper: RW<u32>,
    pub chkr0_clka_lower: RW<u32>,
    pub chkr0_clkb_cnt: RW<u32>,
    pub chkr0_ctrl: RW<u32>,
    pub chkr1_clka_upper: RW<u32>,
    pub chkr1_clka_lower: RW<u32>,
    pub chkr1_clkb_cnt: RW<u32>,
    pub chkr1_ctrl: RW<u32>,
    pub chkr2_clka_upper: RW<u32>,
    pub chkr2_clka_lower: RW<u32>,
    pub chkr2_clkb_cnt: RW<u32>,
    pub chkr2_ctrl: RW<u32>,
    pub chkr3_clka_upper: RW<u32>,
    pub chkr3_clka_lower: RW<u32>,
    pub chkr3_clkb_cnt: RW<u32>,
    pub chkr3_ctrl: RW<u32>,
    pub chkr4_clka_upper: RW<u32>,
    pub chkr4_clka_lower: RW<u32>,
    pub chkr4_clkb_cnt: RW<u32>,
    pub chkr4_ctrl: RW<u32>,
    pub chkr5_clka_upper: RW<u32>,
    pub chkr5_clka_lower: RW<u32>,
    pub chkr5_clkb_cnt: RW<u32>,
    pub chkr5_ctrl: RW<u32>,
    pub chkr6_clka_upper: RW<u32>,
    pub chkr6_clka_lower: RW<u32>,
    pub chkr6_clkb_cnt: RW<u32>,
    pub chkr6_ctrl: RW<u32>,
    pub chkr7_clka_upper: RW<u32>,
    pub chkr7_clka_lower: RW<u32>,
    pub chkr7_clkb_cnt: RW<u32>,
    pub chkr7_ctrl: RW<u32>,
    unused12: [u32; 8],
    pub boot_mode_user: RW<u32>,
    pub boot_mode: BootMode,
    unused13: [u32; 4],
    pub reset_ctrl: ResetCtrl,
    pub blockonly_rst: RW<u32>, // todo: WTC LSB
    pub reset_reason: RW<u32>,  // todo: WTC 0:6
    unused14: [u32; 3],
    pub gem_rst_ctrl: GemRstCtrl,
    unused15: [u32; 1],
    pub peri_rst_ctrl: PeriRstCtrl,
    pub rst_lpd_top: RstLpdTop,
    pub rst_lpd_dbg: RW<u32>,
    unused16: [u32; 3],
    pub boot_pin_ctrl: RW<u32>, // todo: RO 4:7
    unused17: [u32; 7],
    pub bank3_drive0: RW<u32>,
    pub bank3_drive1: RW<u32>,
    pub bank3_input_ctrl: RW<u32>,
    pub bank3_pull_ctrl: RW<u32>,
    pub bank3_pull_enable: RW<u32>,
    pub bank3_slew_ctrl: RW<u32>,
    pub bank3_status: RO<u32>,
}
register_at!(RegisterBlock, 0xFF5E_0000, slcr);

impl Unlocked for RegisterBlock {
    fn unlocked<F: FnMut(&mut Self) -> R, R>(mut f: F) -> R {
        let mut self_ = Self::slcr();
        self_.crl_wprot.write(WProt::zeroed().active(false));
        let r = f(&mut self_);
        self_.crl_wprot.write(WProt::zeroed().active(true));
        r
    }
}

impl RegisterBlock {
    pub fn pre_pll_init(&mut self) {
        // self.ps_sysmon_clk_ctrl.write(
        //     PsSysmonClkCtrl::zeroed()
        //         .divisor1(1)
        //         .divisor0(35)
        //         .srcsel(RpuClkSource::IoPll)
        //         .clkact(true)
        // );
        self.peri_rst_ctrl.modify(|_, w| w.qspi_rst(true));
    }
}

register!(pll_status, PllStatus, RO, u32);
register_bit!(pll_status, rpu_pll_stable, 4);
register_bit!(pll_status, io_pll_stable, 3);
register_bit!(pll_status, rpu_pll_lock, 1);
register_bit!(pll_status, io_pll_lock, 0);

register!(pll_to_fpd_ctrl, PllToFpdCtrl, RW, u32);
register_bits!(pll_to_fpd_ctrl, divisor0, u8, 8, 13);

register!(gem_clk_ctrl, GemClkCtrl, RW, u32);
register_bit!(gem_clk_ctrl, rx_clkact, 26);
register_bit!(gem_clk_ctrl, clkact, 25);
register_bits!(gem_clk_ctrl, divisor1, u8, 16, 21);
register_bits!(gem_clk_ctrl, divisor0, u8, 8, 13);
register_bits_typed!(gem_clk_ctrl, srcsel, u8, IoClkSource, 0, 2);

register!(usb_clk_ctrl, UsbClkCtrl, RW, u32);
register_bit!(usb_clk_ctrl, clkact, 25);
register_bits!(usb_clk_ctrl, divisor1, u8, 16, 21);
register_bits!(usb_clk_ctrl, divisor0, u8, 8, 13);
register_bits_typed!(usb_clk_ctrl, srcsel, u8, IoClkSource, 0, 2);

macro_rules! dual_div_clk_reg {
    ($mod_name: ident, $struct_name: ident, $srcsel_type: ident) => {
        register!($mod_name, $struct_name, RW, u32);
        register_bit!($mod_name, clkact, 24);
        register_bits!($mod_name, divisor1, u8, 16, 21);
        register_bits!($mod_name, divisor0, u8, 8, 13);
        register_bits_typed!($mod_name, srcsel, u8, $srcsel_type, 0, 2);
    };
}

dual_div_clk_reg!(qspi_clk_ctrl, QSpiClkCtrl, IoClkSource);
dual_div_clk_reg!(sdio_clk_ctrl, SdioClkCtrl, IoClkSource);
dual_div_clk_reg!(uart_clk_ctrl, UartClkCtrl, IoClkSource);
dual_div_clk_reg!(spi_clk_ctrl, SpiClkCtrl, IoClkSource);
dual_div_clk_reg!(can_clk_ctrl, CanClkCtrl, IoClkSource);
dual_div_clk_reg!(nand_clk_ctrl, NandClkCtrl, IoClkSource);
dual_div_clk_reg!(pl_clk_ctrl, PlClkCtrl, IoClkSource);
dual_div_clk_reg!(gem_tsu_clk_ctrl, GemTsuClkCtrl, IoClkSource);
dual_div_clk_reg!(ps_sysmon_clk_ctrl, PsSysmonClkCtrl, RpuClkSource);
dual_div_clk_reg!(i2c_clk_ctrl, I2cClkCtrl, IoClkSource);

register!(rpu_clk_ctrl, RpuClkCtrl, RW, u32);
register_bit!(rpu_clk_ctrl, clkact_core, 25);
register_bit!(rpu_clk_ctrl, clkact, 24);
register_bits!(rpu_clk_ctrl, divisor0, u8, 8, 13);
register_bits_typed!(rpu_clk_ctrl, srcsel, u8, RpuClkSource, 0, 2);

macro_rules! single_div_clk_reg {
    // default to RpuClkSource
    ($mod_name: ident, $struct_name: ident, $srcsel_type: ident) => {
        register!($mod_name, $struct_name, RW, u32);
        register_bit!($mod_name, clkact, 24);
        register_bits!($mod_name, divisor1, u8, 16, 21);
        register_bits!($mod_name, divisor0, u8, 8, 13);
        register_bits_typed!($mod_name, srcsel, u8, $srcsel_type, 0, 2);
    };
}

single_div_clk_reg!(iou_switch_clk_ctrl, IouSwitchClkCtrl, RpuClkSource);
single_div_clk_reg!(csu_clk_ctrl, CsuPllCtrl, IoClkSource);
single_div_clk_reg!(pcap_clk_ctrl, PcapClkCtrl, IoClkSource);
single_div_clk_reg!(lpd_switch_clk_ctrl, LpdSwitchClkCtrl, RpuClkSource);
single_div_clk_reg!(lpd_lsbus_clk_ctrl, LpdLsbusClkCtrl, RpuClkSource);
single_div_clk_reg!(dbg_lpd_clk_ctrl, DbgLpdClkCtrl, RpuClkSource);
single_div_clk_reg!(lpd_dma_clk_ctrl, LpdDmaClkCtrl, RpuClkSource);
// todo: timestamp clk can also run directly from PS_REF_CLK (0b1xx)
single_div_clk_reg!(timestamp_clk_ctrl, TimestampClkCtrl, IoClkSource);

register!(pl_thr_ctrl, PlThrCtrl, RW, u32);
register_bits!(pl_thr_ctrl, curr_val, u16, 16, 31, RO);
register_bit!(pl_thr_ctrl, running, 15, RO);
register_bit!(pl_thr_ctrl, cpu_start, 1);
register_bit!(pl_thr_ctrl, cnt_rst, 0);

register!(pl_thr_cnt, PlThrCnt, RW, u32);
register_bits!(pl_thr_cnt, last_cnt, u16, 0, 15);

register!(dll_clk_ctrl, DllClkCtrl, RW, u32);
register_bits!(dll_clk_ctrl, srcsel, u8, 0, 2);

// boot mode pin values read after POR and "triplicated for security"
register!(boot_mode, BootMode, RO, u32);
register_bits!(boot_mode, boot_mode2, u8, 8, 11);
register_bits!(boot_mode, boot_mode1, u8, 4, 7);
register_bits!(boot_mode, boot_mode0, u8, 0, 3);

register!(reset_ctrl, ResetCtrl, RW, u32);
register_bit!(reset_ctrl, soft_reset, 4);

register!(gem_rst_ctrl, GemRstCtrl, RW, u32);
register_bit!(gem_rst_ctrl, gem3_rst, 3);
register_bit!(gem_rst_ctrl, gem2_rst, 2);
register_bit!(gem_rst_ctrl, gem1_rst, 1);
register_bit!(gem_rst_ctrl, gem0_rst, 0);

register!(peri_rst_ctrl, PeriRstCtrl, RW, u32);
register_bit!(peri_rst_ctrl, timestamp_rst, 20);
register_bit!(peri_rst_ctrl, iou_cc_rst, 19);
register_bit!(peri_rst_ctrl, gpio_rst, 18);
register_bit!(peri_rst_ctrl, lpd_dma_rst, 17);
register_bit!(peri_rst_ctrl, nand_rst, 16);
register_bit!(peri_rst_ctrl, swdt_rst, 15);
register_bit!(peri_rst_ctrl, ttc3_rst, 14);
register_bit!(peri_rst_ctrl, ttc2_rst, 13);
register_bit!(peri_rst_ctrl, ttc1_rst, 12);
register_bit!(peri_rst_ctrl, ttc0_rst, 11);
register_bit!(peri_rst_ctrl, i2c1_rst, 10);
register_bit!(peri_rst_ctrl, i2c0_rst, 9);
register_bit!(peri_rst_ctrl, can1_rst, 8);
register_bit!(peri_rst_ctrl, can0_rst, 7);
register_bit!(peri_rst_ctrl, sdio1_rst, 6);
register_bit!(peri_rst_ctrl, sdio0_rst, 5);
register_bit!(peri_rst_ctrl, spi1_rst, 4);
register_bit!(peri_rst_ctrl, spi0_rst, 3);
register_bit!(peri_rst_ctrl, uart1_rst, 2);
register_bit!(peri_rst_ctrl, uart0_rst, 1);
register_bit!(peri_rst_ctrl, qspi_rst, 0);

register!(rst_lpd_top, RstLpdTop, RW, u32);
register_bit!(rst_lpd_top, fpd_rst, 23);
register_bit!(rst_lpd_top, lpd_swdt_rst, 20);
register_bit!(rst_lpd_top, s_axi_lpd_rst, 19);
register_bit!(rst_lpd_top, sysmon_rst, 17);
register_bit!(rst_lpd_top, rtc_rst, 16);
register_bit!(rst_lpd_top, apm_rst, 15);
register_bit!(rst_lpd_top, ipi_rst, 14);
register_bit!(rst_lpd_top, usb1_apb_rst, 11);
register_bit!(rst_lpd_top, usb0_apb_rst, 10);
register_bit!(rst_lpd_top, usb1_hiber_rst, 9);
register_bit!(rst_lpd_top, usb0_hiber_rst, 8);
register_bit!(rst_lpd_top, usb1_core_rst, 7);
register_bit!(rst_lpd_top, usb0_core_rst, 6);
register_bit!(rst_lpd_top, rpu_pge_rst, 4);
register_bit!(rst_lpd_top, ocm_rst, 3);
register_bit!(rst_lpd_top, rpu_amba_rst, 2);
register_bit!(rst_lpd_top, rpu_core1_rst, 1);
register_bit!(rst_lpd_top, rpu_core0_rst, 0);
