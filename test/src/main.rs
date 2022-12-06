
// Adapted from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 5a8d714627
// File: experiments/src/main.rs
// Original author: Astro
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]
// #![feature(compiler_builtins_lib)]
// #![feature(never_type)]

// extern crate alloc;

// use core::mem::uninitialized;
use core::fmt::Write;

use libregister::{RegisterR, RegisterW};
use panic_abort as _;
use r0::zero_bss;

use libboard_zynq_us::{
    uart,
};
use libcortex_a53::{
    asm, cache,
    regs::{MPIDREL1, SP},
};

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
    static mut __stack0_start: u64;
}

#[link_section = ".text.boot"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _boot_cores() -> ! {
    asm!(
        "mrs x0, mpidr_el1",  // read MPIDR_EL1
        "and x0, x0, #0xff",  // get CPU ID within cluster (0-3)
        // TODO: configure SPs for other cores. For now just worry about core 0.
        "cbnz x0, 0f",
        // core 0
        "ldr x1, =__stack0_start",
        "mov sp, x1",
        "bl boot_core0",
        // cores 1-3
        "0:",
        "wfe",
        "b 0b",
        options(noreturn)
    );
}

#[no_mangle]
#[inline(never)]
unsafe fn boot_core0() -> ! {
    // cache_init();
    zero_bss(&mut __bss_start, &mut __bss_end);
    main();
    panic!("return from main")
}

fn cache_init() {
    cache::tlbiall_e3();
    cache::iciallu();
    cache::dcci_all();
    asm::dsb_sys();
    asm::isb();
}

fn main() {
    let mut uart = uart::Uart::uart0(115_200);
    write!(uart, "Hello, world!\r\n");
}
