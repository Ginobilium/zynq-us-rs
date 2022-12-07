// Copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libboard_zynq/src/uart/baud_rate_gen.rs
// Original authors: Astro, Harry Ho, Sebastien Bourdeauducq
// Modifications made for different clock sources, FDIV params, PLL configuration, and SLCRs

use crate::slcr::common::{PllCfg, PllCtrl, PllFracCfg, Unlocked};
use crate::slcr::{crf_apb, crl_apb};
use libregister::{RegisterR, RegisterRW};

#[cfg(feature = "target_zcu111")]
pub const PS_REF_CLK: u32 = 33_333_000;
// ALT_REF_CLK: MIO pin 28 or 51, pins used otherwise on ZCU111
// VIDEO_REF_CLK: MIO pin 27 or 50, pins used otherwise on ZCU111
// AUX_REF_CLK: from PL
// GTR_REF_CLK

// DS926 Table: PS PLL Switching Characteristics (same for both speed grades)
// const PS_PLL_MAX_LOCK_TIME: f32 = 100e-6; // 100 us
const PS_PLL_MAX_OUT_FREQ: u32 = 1_600_000_000;
const PS_PLL_MIN_OUT_FREQ: u32 = 750_000_000;
// const PS_PLL_MAX_VCO_FREQ: u32 = 3_000_000_000;
const PS_PLL_MIN_VCO_FREQ: u32 = 1_500_000_000;

/// UG1085 table 37-1
/// (pll_fdiv_max, (pll_cp, pll_res, lfhf, lock_dly, lock_cnt))
const PLL_FDIV_LOCK_PARAM: &[(u8, (u8, u8, u8, u8, u16))] = &[
    (25, (3, 10, 3, 63, 1000)),
    (26, (3, 10, 3, 63, 1000)),
    (27, (4, 6, 3, 63, 1000)),
    (28, (4, 6, 3, 63, 1000)),
    (29, (4, 6, 3, 63, 1000)),
    (30, (4, 6, 3, 63, 1000)),
    (31, (6, 1, 3, 63, 1000)),
    (32, (6, 1, 3, 63, 1000)),
    (33, (4, 10, 3, 63, 1000)),
    (34, (5, 6, 3, 63, 1000)),
    (35, (5, 6, 3, 63, 1000)),
    (36, (5, 6, 3, 63, 1000)),
    (37, (5, 6, 3, 63, 1000)),
    (38, (5, 6, 3, 63, 975)),
    (39, (3, 12, 3, 63, 950)),
    (40, (3, 12, 3, 63, 925)),
    (41, (3, 12, 3, 63, 900)),
    (42, (3, 12, 3, 63, 875)),
    (43, (3, 12, 3, 63, 850)),
    (44, (3, 12, 3, 63, 850)),
    (45, (3, 12, 3, 63, 825)),
    (46, (3, 12, 3, 63, 800)),
    (47, (3, 12, 3, 63, 775)),
    (48, (3, 12, 3, 63, 775)),
    (49, (3, 12, 3, 63, 750)),
    (50, (3, 12, 3, 63, 750)),
    (51, (3, 2, 3, 63, 725)),
    (52, (3, 2, 3, 63, 700)),
    (53, (3, 2, 3, 63, 700)),
    (54, (3, 2, 3, 63, 675)),
    (55, (3, 2, 3, 63, 675)),
    (56, (3, 2, 3, 63, 650)),
    (57, (3, 2, 3, 63, 650)),
    (58, (3, 2, 3, 63, 625)),
    (59, (3, 2, 3, 63, 625)),
    (60, (3, 2, 3, 63, 625)),
    (82, (3, 2, 3, 63, 600)),  // 61-82
    (102, (4, 2, 3, 63, 600)), // 83-102
    (103, (5, 2, 3, 63, 600)),
    (104, (5, 2, 3, 63, 600)),
    (105, (5, 2, 3, 63, 600)),
    (106, (5, 2, 3, 63, 600)),
    (125, (3, 4, 3, 63, 600)), // 107-125
];

pub trait ClockSource<T: Unlocked> {
    /// picks this ClockSource's registers from the SLCR block
    fn pll_ctrl_regs(slcr: &mut T) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg);

    /// query PLL lock status
    fn pll_locked() -> bool;

    // todo: is there any situation in which we'll actually want to use fractional mode?
    /// query fraction mode enable bit
    fn frac_enabled(pll_frac_cfg: &mut PllFracCfg) -> bool {
        bool::from(pll_frac_cfg.read().enabled())
    }

    /// get configured frequency
    fn freq(pll_ctrl: &mut PllCtrl) -> u32 {
        // todo: take into account fractional part (if enabled)
        let ctrl = pll_ctrl.read();
        let (mul, div) = if ctrl.pll_bypass() {
            // todo: should technically read POST_SRC field to get src
            (1u32, 1u32)
        } else {
            // same as above, but PRE_SRC
            (u32::from(ctrl.pll_fdiv()), u32::from(ctrl.pll_div2()) + 1)
        };
        mul * PS_REF_CLK / div
    }

    fn name() -> &'static str;

    // UG1085 Chapter 37: PS Clock Subsystem
    fn setup(target_freq: u32) {
        assert!(target_freq >= PS_PLL_MIN_OUT_FREQ && target_freq <= PS_PLL_MAX_OUT_FREQ);
        let div2 = target_freq <= PS_PLL_MIN_VCO_FREQ;
        let divisor = u32::from(div2) + 1;
        let fdiv = (target_freq * divisor / PS_REF_CLK).min(125) as u8;
        let (pll_cp, pll_res, lfhf, lock_dly, lock_cnt) = PLL_FDIV_LOCK_PARAM
            .iter()
            .filter(|(fdiv_max, _)| fdiv <= *fdiv_max)
            .nth(0)
            .expect("PLL_FDIV_LOCK_PARAM")
            .1
            .clone();

        // debug!("Set {} to {} Hz", Self::name(), target_freq);
        T::unlocked(|slcr| {
            let (pll_ctrl, pll_cfg, _) = Self::pll_ctrl_regs(slcr);

            // Write fdiv, div2
            pll_ctrl.modify(|_, w| w.pll_fdiv(fdiv).pll_div2(div2));
            // Configure
            // no need to zero as we're writing every field
            pll_cfg.modify(|_, w| {
                w.lock_dly(lock_dly)
                    .lock_cnt(lock_cnt)
                    .lfhf(lfhf)
                    .pll_cp(pll_cp)
                    .pll_res(pll_res)
            });
            // Bypass
            pll_ctrl.modify(|_, w| w.pll_bypass(true));
            // Reset
            pll_ctrl.modify(|_, w| w.pll_reset(true));
            pll_ctrl.modify(|_, w| w.pll_reset(false));
            // Wait for PLL lock
            // todo: add timeout here according to the 100 us spec?
            while !Self::pll_locked() {}
            // Remove bypass
            pll_ctrl.modify(|_, w| w.pll_bypass(false));
        });
    }
}

/// APU PLL: Recommended clock source for the APUs and the FPD interconnect
pub struct ApuPll;

impl ClockSource<crf_apb::RegisterBlock> for ApuPll {
    #[inline]
    fn pll_ctrl_regs(
        slcr: &mut crf_apb::RegisterBlock,
    ) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg) {
        (
            &mut slcr.apu_pll_ctrl,
            &mut slcr.apu_pll_cfg,
            &mut slcr.apu_pll_frac_cfg,
        )
    }

    #[inline]
    fn pll_locked() -> bool {
        let slcr = crf_apb::RegisterBlock::slcr();
        slcr.pll_status.read().apu_pll_lock()
    }

    fn name() -> &'static str {
        &"APU_PLL"
    }
}

/// DDR PLL: Recommended clock for the DDR DRAM controller and AXI_HP interfaces
pub struct DdrPll;

impl ClockSource<crf_apb::RegisterBlock> for DdrPll {
    #[inline]
    fn pll_ctrl_regs(
        slcr: &mut crf_apb::RegisterBlock,
    ) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg) {
        (
            &mut slcr.ddr_pll_ctrl,
            &mut slcr.ddr_pll_cfg,
            &mut slcr.ddr_pll_frac_cfg,
        )
    }

    #[inline]
    fn pll_locked() -> bool {
        let slcr = crf_apb::RegisterBlock::slcr();
        slcr.pll_status.read().ddr_pll_lock()
    }

    fn name() -> &'static str {
        &"DDR_PLL"
    }
}

/// Video PLL: Recommended clock for DisplayPort
pub struct VideoPll;

impl ClockSource<crf_apb::RegisterBlock> for VideoPll {
    #[inline]
    fn pll_ctrl_regs(
        slcr: &mut crf_apb::RegisterBlock,
    ) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg) {
        (
            &mut slcr.video_pll_ctrl,
            &mut slcr.video_pll_cfg,
            &mut slcr.video_pll_frac_cfg,
        )
    }

    #[inline]
    fn pll_locked() -> bool {
        let slcr = crf_apb::RegisterBlock::slcr();
        slcr.pll_status.read().video_pll_lock()
    }

    fn name() -> &'static str {
        &"VIDEO_PLL"
    }
}

/// I/O PLL: Recommended clock for I/O peripherals
pub struct IoPll;

impl ClockSource<crl_apb::RegisterBlock> for IoPll {
    #[inline]
    fn pll_ctrl_regs(
        slcr: &mut crl_apb::RegisterBlock,
    ) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg) {
        (
            &mut slcr.io_pll_ctrl,
            &mut slcr.io_pll_cfg,
            &mut slcr.io_pll_frac_cfg,
        )
    }

    #[inline]
    fn pll_locked() -> bool {
        let slcr = crl_apb::RegisterBlock::slcr();
        slcr.pll_status.read().io_pll_lock()
    }

    fn name() -> &'static str {
        &"IO_PLL"
    }
}

/// RPU PLL: Recommended clock for RPUs and LPD interconnect
pub struct RpuPll;

impl ClockSource<crl_apb::RegisterBlock> for RpuPll {
    #[inline]
    fn pll_ctrl_regs(
        slcr: &mut crl_apb::RegisterBlock,
    ) -> (&mut PllCtrl, &mut PllCfg, &mut PllFracCfg) {
        (
            &mut slcr.io_pll_ctrl,
            &mut slcr.io_pll_cfg,
            &mut slcr.io_pll_frac_cfg,
        )
    }

    #[inline]
    fn pll_locked() -> bool {
        let slcr = crl_apb::RegisterBlock::slcr();
        slcr.pll_status.read().rpu_pll_lock()
    }

    fn name() -> &'static str {
        &"RPU_PLL"
    }
}
