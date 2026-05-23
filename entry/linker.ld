ENTRY(asm_entry)
OUTPUT_ARCH(riscv)

MEMORY {

    ram (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

PHDRS {
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS {
  . = ORIGIN(ram);
  .text : {
    . = ALIGN(8);
    KEEP(*(.text.init))
    *(.text*)   
  } >ram AT>ram :text

  .rodata : { 
    *(.rodata .rodata.*)
  } >ram AT>ram :text 

  .data : { 
    *(.sdata .sdata.*) *(.data .data.*)
  } >ram AT>ram :data 

  .bss :{ 
    PROVIDE(_bss_start = .); 
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .); 
  } >ram AT>ram :bss 

}