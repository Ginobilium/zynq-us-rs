use libregister::{register_bit, register_bits, RegisterR, RegisterW};

// Macros copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libcortex_a9/src/regs.rs
// Original authors: Astro, Sebastien Bourdeauducq, pca006132
// Modified to use `asm!` instead of `llvm_asm!` and 64-bit regs
macro_rules! def_reg_r {
    ($name: ident, $type: ty, $asm_instr: tt) => {
        impl RegisterR for $name {
            type R = $type;

            #[inline]
            fn read(&self) -> Self::R {
                let mut value: $type;
                unsafe { asm!($asm_instr, out(reg) value) }
                value.into()
            }
        }
    };

    ($name: ident, $type: ty, $inner: ty, $asm_instr: tt) => {
        impl RegisterR for $name {
            type R = $type;

            #[inline]
            fn read(&self) -> Self::R {
                let mut value: $inner;
                unsafe { asm!($asm_instr, out(reg) value) }
                value.into()
            }
        }
    };
}

macro_rules! def_reg_w {
    ($name: ident, $type: ty, $asm_instr: tt) => {
        impl RegisterW for $name {
            type W = $type;

            #[inline]
            fn write(&mut self, value: Self::W) {
                let value: $type = value.into();
                unsafe { asm!($asm_instr, in(reg) value) }
            }

            #[inline]
            fn zeroed() -> Self::W {
                (0 as $type).into()
            }
        }
    };

    ($name: ident, $type: ty, $inner: ty, $asm_instr: tt) => {
        impl RegisterW for $name {
            type W = $type;

            #[inline]
            fn write(&mut self, value: Self::W) {
                let value: $inner = value.into();
                unsafe { asm!($asm_instr, in(reg) value) }
            }

            #[inline]
            fn zeroed() -> Self::W {
                (0 as $inner).into()
            }
        }
    };
}

macro_rules! wrap_reg {
    ($mod_name: ident, $inner: ty) => {
        pub mod $mod_name {
            pub struct Read {
                pub inner: $inner,
            }
            impl From<$inner> for Read {
                #[inline]
                fn from(value: $inner) -> Self {
                    Read { inner: value }
                }
            }

            pub struct Write {
                pub inner: $inner,
            }
            impl From<$inner> for Write {
                #[inline]
                fn from(value: $inner) -> Self {
                    Write { inner: value }
                }
            }
            impl Into<$inner> for Write {
                #[inline]
                fn into(self) -> $inner {
                    self.inner
                }
            }
        }
    };
}

/// Stack Pointer
pub struct SP;
def_reg_r!(SP, u64, "mov {0}, sp");
def_reg_w!(SP, u64, "mov sp, {0}");

/// Multiprocessor Affinity Register
pub struct MPIDREL1;
wrap_reg!(mpidr_el1, u64);
def_reg_r!(MPIDREL1, mpidr_el1::Read, u64, "mrs {0}, mpidr_el1");
register_bit!(mpidr_el1, single_core, 30);
register_bit!(mpidr_el1, mt, 24);
register_bits!(mpidr_el1, aff2, u8, 16, 23);
register_bits!(mpidr_el1, aff1, u8, 8, 15);
register_bits!(mpidr_el1, cpu_id, u8, 0, 7);

/// System Control Register - EL3
pub struct SCTLREL3;
def_reg_r!(SCTLREL3, sctlr_el3::Read, u32, "mrs {0:w}, sctlr_el3");
def_reg_w!(SCTLREL3, sctlr_el3::Write, u32, "msr sctlr_el3, {0:w}");
wrap_reg!(sctlr_el3, u32);
register_bit!(sctlr_el3, ee, 25);
register_bit!(sctlr_el3, wxn, 19);
register_bit!(sctlr_el3, i, 12);
register_bit!(sctlr_el3, sa, 3);
register_bit!(sctlr_el3, c, 2);
register_bit!(sctlr_el3, a, 1);
register_bit!(sctlr_el3, m, 0);
