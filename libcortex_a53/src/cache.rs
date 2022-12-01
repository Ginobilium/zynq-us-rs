///! Cache maintenance operations
use core::mem::size_of_val;

use super::asm::{dmb_os, dsb_os};

// Same for all Cortex-A53s (L1 refers to the D-Cache)
const L1_NWAYS: usize = 4;
const L1_BIT_POS_OF_WAY: usize = 30; // 32 - log2(L1_NWAYS)
const LINELEN: usize = 64; // L1 and L2
const LINE_MASK: usize = LINELEN - 1;
const L2_NWAYS: usize = 16;
const L2_BIT_POS_OF_WAY: usize = 28; // 32 - log2(L2_ASSOC)
const BIT_POS_OF_SET: usize = 6; // log2(LINELEN)
                                 // Zynq US+ specific
                                 // const L1_SIZE: usize = 32_768;  // 32 KB
const L1_NSETS: usize = 128; // L1_SIZE / (L1D_ASSOC * LINELEN)
                             // const L2_SIZE: usize = 1_048_576;  // 1 MB
const L2_NSETS: usize = 1024; // L2_SIZE / (L2_ASSOC * LINELEN)

/// Level field for Set/Way-based instructions (Zynq US+ has no L3 Cache)
pub enum CacheLevel {
    L1 = 0b00,
    L2 = 0b01,
}

#[inline(always)]
pub fn translate_set_way_level(set: usize, way: usize, level: CacheLevel) -> usize {
    let (nsets, nways, bit_pos_of_way) = match level {
        CacheLevel::L1 => (L1_NSETS, L1_NWAYS, L1_BIT_POS_OF_WAY),
        CacheLevel::L2 => (L2_NSETS, L2_NWAYS, L2_BIT_POS_OF_WAY),
    };
    assert!(set < nsets, "Invalid set provided");
    assert!(way < nways, "Invalid way provided");
    (set << BIT_POS_OF_SET) | (way << bit_pos_of_way) | ((level as usize) << 1)
}

/// Instruction cache invalidate all to PoU Inner Shareable
#[inline(always)]
pub fn icialluis() {
    unsafe { asm!("ic ialluis") }
}

/// Instruction cache invalidate all to PoU
#[inline(always)]
pub fn iciallu() {
    unsafe { asm!("ic iallu") }
}

/// Instruction cache invalidate by virtual address (VA) to PoU
#[inline(always)]
pub fn icivau(addr: usize) {
    unsafe { asm!("ic ivau, {0}", in(reg) addr) }
}

/// Data cache invalidate by VA to PoC
/// Unsafe as it invalidates the entire line containing the address
#[inline(always)]
pub unsafe fn dcivac(addr: usize) {
    asm!("dc ivac, {0}", in(reg) addr)
}

/// Data cache invalidate by set/way
#[inline(always)]
pub fn dcisw(set: usize, way: usize, level: CacheLevel) {
    unsafe { asm!("dc isw, {0}", in(reg) translate_set_way_level(set, way, level)) }
}

/// Data cache clean by set/way
#[inline(always)]
pub fn dccsw(set: usize, way: usize, level: CacheLevel) {
    unsafe { asm!("dc csw, {0}", in(reg) translate_set_way_level(set, way, level)) }
}

/// Data cache clean and invalidate by set/way
#[inline(always)]
pub fn dccisw(set: usize, way: usize, level: CacheLevel) {
    unsafe { asm!("dc cisw, {0}", in(reg) translate_set_way_level(set, way, level)) }
}

/// Data cache clean by VA to PoC
#[inline(always)]
pub fn dccvac(addr: usize) {
    unsafe { asm!("dc cvac, {0}", in(reg) addr) }
}

/// Data cache clean by VA to PoU
#[inline(always)]
pub fn dccvau(addr: usize) {
    unsafe { asm!("dc cvau, {0}", in(reg) addr) }
}

/// Data cache clean and invalidate by VA to PoC
#[inline(always)]
pub fn dccivac(addr: usize) {
    unsafe { asm!("dc civac, {0}", in(reg) addr) }
}

#[inline(always)]
pub fn dci_all() {
    dmb_os();
    for set in 0..L2_NSETS {
        for way in 0..L2_NWAYS {
            dcisw(set, way, CacheLevel::L2);
        }
    }
    for set in 0..L1_NSETS {
        for way in 0..L1_NWAYS {
            dcisw(set, way, CacheLevel::L1);
        }
    }
}

#[inline(always)]
pub fn dcc_all() {
    dmb_os();
    for set in 0..L2_NSETS {
        for way in 0..L2_NWAYS {
            dccsw(set, way, CacheLevel::L2);
        }
    }
    for set in 0..L1_NSETS {
        for way in 0..L1_NWAYS {
            dccsw(set, way, CacheLevel::L1);
        }
    }
    dsb_os();
}

#[inline(always)]
pub fn dcci_all() {
    dmb_os();
    for set in 0..L2_NSETS {
        for way in 0..L2_NWAYS {
            dccisw(set, way, CacheLevel::L2);
        }
    }
    for set in 0..L1_NSETS {
        for way in 0..L1_NWAYS {
            dccisw(set, way, CacheLevel::L1);
        }
    }
    dsb_os();
}

#[inline]
fn cache_line_addrs(
    start_addr: usize,
    end_addr: usize,
    strict_align: bool,
) -> impl Iterator<Item = usize> {
    if strict_align {
        assert_eq!(start_addr & LINE_MASK, 0, "Start address is not aligned");
        assert_eq!(
            end_addr & LINE_MASK,
            LINE_MASK,
            "End address is not aligned"
        );
    }
    let start_line_addr = start_addr & !LINE_MASK;

    (start_line_addr..=end_addr).step_by(LINELEN)
}

#[inline]
fn object_cache_line_addrs<T>(object: &T, strict_align: bool) -> impl Iterator<Item = usize> {
    let start_addr = object as *const _ as usize;
    let end_addr = start_addr + size_of_val(object) - 1;
    cache_line_addrs(start_addr, end_addr, strict_align)
}

#[inline]
fn slice_cache_line_addrs<T>(slice: &[T], strict_align: bool) -> impl Iterator<Item = usize> {
    let start_addr = &slice[0] as *const _ as usize;
    let end_addr = start_addr + size_of_val(slice) - 1;
    cache_line_addrs(start_addr, end_addr, strict_align)
}

/// Invalidate all stage 1 translations used at EL1
#[inline(always)]
pub fn tlbiall_e1() {
    unsafe { asm!("tlbi alle1") }
}

/// Invalidate all stage 1 translations used at EL2
#[inline(always)]
pub fn tlbiall_e2() {
    unsafe { asm!("tlbi alle2") }
}

/// Invalidate all stage 1 translations used at EL3
#[inline(always)]
pub fn tlbiall_e3() {
    unsafe { asm!("tlbi alle3") }
}
