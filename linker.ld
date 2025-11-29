ENTRY(_start)

SECTIONS {
    . = 0x80000;

    .text : {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
        . = ALIGN(8);
    }

    /DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
    }
}
