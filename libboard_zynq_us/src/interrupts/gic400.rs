//! APU Interrupts - GIC400

use libregister::register_at;
use volatile_register::{RO, RW, WO};

/// GIC distributor registers
#[repr(C)]
pub struct GicD {
    pub ctrl: RW<u32>,
    pub type_r: RO<u32>,
    pub iid: RO<u32>,
    unused0: [u32; 29],
    pub group: [RW<u32>; 6],
    unused1: [u32; 26],
    pub set_enable: [RW<u32>; 6],
    unused2: [u32; 26],
    pub clear_enable: [RW<u32>; 6],
    unused3: [u32; 26],
    pub set_pending: [RW<u32>; 6],
    unused4: [u32; 26],
    pub clear_pending: [RW<u32>; 6],
    unused5: [u32; 26],
    pub set_active: [RW<u32>; 6],
    unused6: [u32; 26],
    pub clear_active: [RW<u32>; 6],
    unused7: [u32; 26],
    pub priority: [RW<u32>; 48],
    unused8: [u32; 208],
    // GICv2 spec: "In a multiprocessor implementation, GICD_ITARGETSR0 to GICD_ITARGETSR7 are
    // banked for each connected processor. These registers hold the CPU targets fields for
    // interrupts 0-31"
    // Field names aren't pretty but I want to make the base index explicit
    pub target_0_7: [RO<u32>; 8],
    pub target_8_47: [RW<u32>; 40],
    unused9: [u32; 208],
    pub config_sgi: RO<u32>,
    pub config_ppi: RO<u32>,
    pub config_spi: [RW<u32>; 10],
    unused10: [u32; 52],
    pub ppi_status: RO<u32>,
    pub spi_status: [RO<u32>; 5],
    unused11: [u32; 122],
    pub sgi: WO<u32>,
    unused12: [u32; 3],
    pub sgi_clear_pending: [RW<u32>; 4],
    pub sgi_set_pending: [RW<u32>; 4],
    unused13: [u32; 40],
    pub pid4: RO<u32>,
    pub pid5: RO<u32>,
    pub pid6: RO<u32>,
    pub pid7: RO<u32>,
    pub pid0: RO<u32>,
    pub pid1: RO<u32>,
    pub pid2: RO<u32>,
    pub pid3: RO<u32>,
    pub cid: [RO<u32>; 4],
}

register_at!(GicD, 0xF901_0000, gicd);

/// GIC CPU interface register5s
#[repr(C)]
pub struct GicC {
    pub ctrl: RW<u32>,
    pub prio_mask: RW<u32>,
    pub binary_point: RW<u32>,
    pub interrupt_ack: RO<u32>,
    pub end_of_interrupt: WO<u32>,
    pub running_prio: RO<u32>,
    pub highest_prio_pending: RO<u32>,
    pub aliased_binary_point: RW<u32>,
    pub aliased_interrupt_ack: RO<u32>,
    pub aliased_end_of_interrupt: WO<u32>,
    pub aliased_highest_prio_pending: RO<u32>,
    unused0: [u32; 41],
    pub active_prio: RW<u32>,
    unused1: [u32; 3],
    pub nonsecure_active_prio: RW<u32>,
    unused2: [u32; 6],
    pub iid: RO<u32>,
}

register_at!(GicC, 0xF902_0000, gicc);
