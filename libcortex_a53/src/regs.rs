use libregister::{register_bit, register_bits, RegisterR, RegisterW};

// Macros copied from https://git.m-labs.hk/M-labs/zynq-rs
// Commit: 0a3a777652
// File: libcortex_a9/src/regs.rs
// Original authors: Astro, Sebastien Bourdeauducq, pca006132
// Modified to use `asm!` instead of `llvm_asm!` and 64-bit regs
macro_rules! def_reg_r {
    ($name:tt, $type: ty, $asm_instr:tt) => {
        impl RegisterR for $name {
            type R = $type;

            #[inline]
            fn read(&self) -> Self::R {
                let mut value: u64;
                unsafe { asm!($asm_instr, out(reg) value) }
                value.into()
            }
        }
    }
}

macro_rules! def_reg_w {
    ($name:ty, $type:ty, $asm_instr:tt) => {
        impl RegisterW for $name {
            type W = $type;

            #[inline]
            fn write(&mut self, value: Self::W) {
                let value: u64 = value.into();
                unsafe { asm!($asm_instr, in(reg) value) }
            }

            #[inline]
            fn zeroed() -> Self::W {
                0u64.into()
            }
        }
    }
}

macro_rules! wrap_reg {
    ($mod_name: ident) => {
        pub mod $mod_name {
            pub struct Read {
                pub inner: u64,
            }
            impl From<u64> for Read {
                #[inline]
                fn from(value: u64) -> Self {
                    Read { inner: value }
                }
            }

            pub struct Write {
                pub inner: u64,
            }
            impl From<u64> for Write {
                #[inline]
                fn from(value: u64) -> Self {
                    Write { inner: value }
                }
            }
            impl Into<u64> for Write {
                #[inline]
                fn into(self) -> u64 {
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
def_reg_r!(MPIDREL1, mpidr_el1::Read, "mrs {0}, mpidr_el1");
wrap_reg!(mpidr_el1);
register_bit!(mpidr_el1, single_core, 30);
register_bit!(mpidr_el1, mt, 24);
register_bits!(mpidr_el1, aff2, u8, 16, 23);
register_bits!(mpidr_el1, aff1, u8, 8, 15);
register_bits!(mpidr_el1, cpu_id, u8, 0, 7);
