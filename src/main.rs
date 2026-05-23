#![no_std]
#![no_main]

mod panic_handler;
mod uart;
use core::arch::global_asm;
use core::include_str;
global_asm!(include_str!("../entry/entry.S"));

#[unsafe(no_mangle)]
fn r_entry() {
    //::write_text("Yo\n");
    loop {};
    panic!();
}


