    .section .text._start
    .globl _start

_start:
    // Set stack pointer (temporary)
    ldr x0, =_stack_start
    mov sp, x0

    // Jump to Rust entrypoint
    bl kernel_main

1:  b 1b     // infinite loop

    .align 16
_stack_start:
