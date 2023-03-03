//! Internal utility functions

pub fn div_round_closest(q: u32, d: u32) -> u32 {
    (q + (d / 2)) / d
}
