    .section .text._start
    .globl _start

_start:
    // Load address of stack_top and set SP there
    ldr x0, =stack_top
    mov sp, x0

    // Debug: write 'A' directly to UART
    mov x1, #0x41            // ASCII 'A'
    ldr x0, =0x09000000      // PL011 UART base
    strb w1, [x0]            // write to DR register

    // Call Rust kernel entry
    bl kernel_main

1:
    b 1b


    .section .bss
    .align 16
stack:
    .skip 8192        // 8 KB stack
stack_top:
