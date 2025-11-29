ENTRY(_start)

SECTIONS
{
    . = 0x40000000;      /* QEMU virt loads -kernel here */

    /* ------------------------------------------------- */
    /* TEXT: pull in entry code + all kernel code        */
    /* ------------------------------------------------- */
    .text : ALIGN(4K) {
        KEEP(*(.text._start))   /* <- pulls entry.S */
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

    /* ------------------------------------------------- */
    /* Start heap AFTER bss                              */
    /* ------------------------------------------------- */
    _heap_start = ALIGN(4K);
}
