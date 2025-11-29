ENTRY(_start)

SECTIONS
{
    . = 0x40000000;  /* QEMU loads kernel here */

    .text : ALIGN(4K) {
        KEEP(*(.text._start))
        *(.text*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata*)
    }

    .data : ALIGN(4K) {
        *(.data*)
    }

    .bss (NOLOAD) : ALIGN(4K) {
        *(.bss*)
        *(COMMON)
    }

    _heap_start = ALIGN(4K);
}
