ENTRY(vector_table);

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

    .stack0 (NOLOAD) : ALIGN(64) {
        __stack0_end = .;
        . = ORIGIN(OCM) + LENGTH(OCM) - 64;
        __stack0_start = .;
    } > OCM

    /DISCARD/ :
    {
        /* Unused exception related info that only wastes space */
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
