///! Type definitions for re-use across SLCR blocks
use libregister::{register, register_bit, register_bits};

pub trait Unlocked {
    fn unlocked<F: FnMut(&mut Self) -> R, R>(f: F) -> R;
}

register!(wprot, WProt, RW, u32);
register_bit!(wprot, active, 0);

register!(pll_ctrl, PllCtrl, RW, u32);
register_bits!(pll_ctrl, pll_post_src, u8, 24, 26);
register_bits!(pll_ctrl, pll_pre_src, u8, 20, 22);
register_bit!(pll_ctrl, pll_div2, 16);
register_bits!(pll_ctrl, pll_fdiv, u8, 8, 14);
register_bit!(pll_ctrl, pll_bypass, 3);
register_bit!(pll_ctrl, pll_reset, 0);

register!(pll_cfg, PllCfg, RW, u32);
register_bits!(pll_cfg, lock_dly, u8, 25, 31);
register_bits!(pll_cfg, lock_cnt, u16, 13, 22);
register_bits!(pll_cfg, lfhf, u8, 10, 11);
register_bits!(pll_cfg, pll_cp, u8, 5, 8);
register_bits!(pll_cfg, pll_res, u8, 0, 3);

register!(pll_frac_cfg, PllFracCfg, RW, u32);
register_bit!(pll_frac_cfg, enabled, 31);
register_bits!(pll_frac_cfg, data, u16, 0, 15);
