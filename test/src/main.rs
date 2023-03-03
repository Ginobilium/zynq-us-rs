// Adapted from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 5a8d714627
// File: experiments/src/main.rs
// Original author: Astro
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]

use core::arch::asm;

use log::info;
use r0::zero_bss;

use libboard_zynq_us::{
    clocks, ddr, logger, print, println,
    slcr::{common::Unlocked, crf_apb, crl_apb, iou_slcr},
};
use libcortex_a53::{asm, cache};

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
    static mut __stack0_start: u64;
}

#[link_section = ".text.boot"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _boot_cores() {
    asm!(
        // get CPU ID within cluster (0-3)
        "mrs x0, MPIDR_EL1",
        "and x0, x0, #0xff",
        // TODO: configure SPs for other cores. For now just worry about core 0.
        "cbnz x0, 0f",
        // core 0
        "ldr x1, =vector_table",
        "msr VBAR_EL3, x1",
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
    cache_init();
    enable_fpu();
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

fn enable_fpu() {
    unsafe {
        asm!(
            // disable trapping for all ELs
            "msr CPTR_EL3, xzr",
            "msr CPTR_EL2, xzr",
            "mov {tmp}, #(0x3 << 20)",
            "msr CPACR_EL1, {tmp}",
            "isb",
            tmp = out(reg) _,
        );
    }
}

fn main() {
    // setup MIO pins
    iou_slcr::RegisterBlock::unlocked(|slcr| slcr.mio_init());

    // Initialize PLLs, dividers, source selects, etc.
    clocks::Clocks::init();
    logger::init().unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    info!("Clock initialization complete.");

    let ddr_config = ddr::spd::read_spd_eeprom();
    info!("SPD EEPROM read done.\nConfig: {:?}", ddr_config);

    loop {}
}

#[no_mangle]
#[inline(never)]
pub fn synchronous_handler() {
    println!("Synchronous exception");
    loop {}
}

#[no_mangle]
#[inline(never)]
pub fn irq_handler() {
    println!("IRQ");
    loop {}
}

#[no_mangle]
#[inline(never)]
pub fn fiq_handler() {
    println!("FIQ");
    loop {}
}

#[no_mangle]
#[inline(never)]
pub fn system_error_handler() {
    println!("System error");
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Panic at ");
    if let Some(location) = info.location() {
        print!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        print!("unknown location");
    }
    if let Some(message) = info.message() {
        println!(": {}", message);
    } else {
        println!("");
    }
    loop {}
}

fn test_addrs() {
    let crf_apb = crf_apb::RegisterBlock::crf_apb();
    let mut start = &crf_apb.err_ctrl as *const _ as u32;
    let mut end = &crf_apb.rst_ddr_ss as *const _ as u32;
    assert_eq!(start, 0xFD1A_0000);
    assert_eq!(end, 0xFD1A_0108);

    let crl_apb = crl_apb::RegisterBlock::crl_apb();
    start = &crl_apb.err_ctrl as *const _ as u32;
    end = &crl_apb.bank3_status as *const _ as u32;
    assert_eq!(start, 0xFF5E_0000);
    assert_eq!(end, 0xFF5E_0288);

    let iou_slcr = iou_slcr::RegisterBlock::iou_slcr();
    start = &iou_slcr.mio_pin[0] as *const _ as u32;
    end = &iou_slcr.itr as *const _ as u32;
    assert_eq!(start, 0xFF18_0000);
    assert_eq!(end, 0xFF18_0710);
}
