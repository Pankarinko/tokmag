.section .text.init, "ax"
.globl asm_entry

asm_entry:
    la sp, asm_stacktop
    j r_entry

.section .bss.stack, "aw", @nobits
.align 16

asm_stack:
    .space 1024
asm_stacktop:

