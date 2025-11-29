    .section .text._start
    .globl _start

_start:
    // Load address of stack_top relative to PC
    adrp x0, stack_top
    add  x0, x0, :lo12:stack_top
    mov  sp, x0

    bl kernel_main

1:  b 1b

    .section .bss
    .align 16
stack:
    .skip 8192
stack_top:
