ENTRY(asm_entry)
OUTPUT_ARCH(riscv)

MEMORY {

    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 1M
}

SECTIONS {
  . = 0x80000000;
  .text : {
       . = ALIGN(8);
    *(.text*)
    
 } > RAM
}