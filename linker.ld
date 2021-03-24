ENTRY(_start)

SECTIONS
{
    /* We wanna be placed in the higher half, 2MiB above 0 in physical memory. */
    . = 0xffffffff80200000;

    .stivalehdr : {
        KEEP(*(.stivalehdr))
    }

    /* Then let's place all the other traditional executable sections afterwards. */
    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(COMMON)
        *(.bss*)
    }
}