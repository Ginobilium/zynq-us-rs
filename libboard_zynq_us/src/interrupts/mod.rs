//! Interrupt registers and handling
//! 
//! From UG1085: 
//! > The platform management unit (PMU) and configuration security unit (CSU) have local interrupt controllers.
//! > The RPU uses the Arm PL-390 generic interrupt controller that is compliant to the GICv1
//! > architecture specification. The APU MPCore uses the Arm GIC-400 generic interrupt
//! > controller and is compliant to the GICv2 architecture specification. The GIC manages the
//! > software-generated interrupts (SGI), each CPU’s private peripheral interrupts (PPI), and the
//! > shared peripheral interrupts (SPI).
pub mod gic400;