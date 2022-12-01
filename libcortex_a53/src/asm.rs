///! Miscellaneous instructions

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb") }
}

/// Data Memory Barrier - Full System
#[inline(always)]
pub fn dmb_sys() {
    unsafe { asm!("dmb sy") }
}

/// Data Memory Barrier - Non-Shareable, Read & Write
#[inline(always)]
pub fn dmb_ns() {
    unsafe { asm!("dmb nsh") }
}

/// Data Memory Barrier - Inner Shareable, Read & Write
#[inline(always)]
pub fn dmb_is() {
    unsafe { asm!("dmb ish") }
}

/// Data Memory Barrier - Outer Shareable, Read & Write
#[inline(always)]
pub fn dmb_os() {
    unsafe { asm!("dmb osh") }
}

/// Data Synchronization Barrier - Full System, Read & Write
#[inline(always)]
pub fn dsb_sys() {
    unsafe { asm!("dsb sy") }
}

/// Data Synchronization Barrier - Non-Shareable, Read & Write
#[inline(always)]
pub fn dsb_ns() {
    unsafe { asm!("dsb nsh") }
}

/// Data Synchronization Barrier - Inner Shareable, Read & Write
#[inline(always)]
pub fn dsb_is() {
    unsafe { asm!("dsb ish") }
}

/// Data Synchronization Barrier - Outer Shareable, Read & Write
#[inline(always)]
pub fn dsb_os() {
    unsafe { asm!("dsb osh") }
}

/// No-op
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop") }
}

/// Wait for Event
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe") }
}
