ENTRY(_boot_cores);

/* Provide some defaults */
PROVIDE(Reset = _boot_cores);
PROVIDE(UndefinedInstruction = Reset);
PROVIDE(SoftwareInterrupt = Reset);
PROVIDE(PrefetchAbort = Reset);
PROVIDE(DataAbort = Reset);
PROVIDE(ReservedException = Reset);
PROVIDE(IRQ = Reset);
PROVIDE(FIQ = Reset);

MEMORY
{
    /* 256 kB On-Chip Memory */
    OCM : ORIGIN = 0xFFFC0000, LENGTH = 0x40000
}

SECTIONS
{
    .text :
    {
        KEEP(*(.text.exceptions));
        *(.text.boot);
        *(.text .text.*);
    } > OCM
 
    .rodata : ALIGN(64)
    {
        *(.rodata .rodata.*);
    } > OCM
 
    .data : ALIGN(64)
    {
        *(.data .data.*);
    } > OCM
 
    .bss (NOLOAD) : ALIGN(64)
    {
        __bss_start = .;
        *(.bss .bss.*);
        . = ALIGN(64);
        __bss_end = .;
    } > OCM

    .irq_stack3 (NOLOAD) : ALIGN(64)
    {
        __irq_stack3_end = .;
        . += 0x400;
        __irq_stack3_start = .;
    } > OCM

    .irq_stack2 (NOLOAD) : ALIGN(64)
    {
        __irq_stack2_end = .;
        . += 0x400;
        __irq_stack2_start = .;
    } > OCM

    .irq_stack1 (NOLOAD) : ALIGN(64)
    {
        __irq_stack1_end = .;
        . += 0x400;
        __irq_stack1_start = .;
    } > OCM

    .irq_stack0 (NOLOAD) : ALIGN(64)
    {
        __irq_stack0_end = .;
        . += 0x400;
        __irq_stack0_start = .;
    } > OCM

    .stack3 (NOLOAD) : ALIGN(64) {
        __stack3_end = .;
        . += 0x800;
        __stack3_start = .;
    } > OCM

    .stack2 (NOLOAD) : ALIGN(64) {
        __stack2_end = .;
        . += 0x800;
        __stack2_start = .;
    } > OCM

    .stack1 (NOLOAD) : ALIGN(64) {
        __stack1_end = .;
        . += 0x800;
        __stack1_start = .;
    } > OCM

    .stack0 (NOLOAD) : ALIGN(64) {
        __stack0_end = .;
        . = ORIGIN(OCM) + LENGTH(OCM) - 8;
        . = ALIGN(64);
        __stack0_start = .;

        /* unused heap0 to prevent the linker from complaining*/
        __heap0_start = .;
        __heap0_end = .;
    } > OCM

    /DISCARD/ :
    {
        /* Unused exception related info that only wastes space */
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
