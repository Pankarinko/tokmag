#![no_std]
#![no_main]
mod panic_handler;
mod uart;

#[unsafe(no_mangle)]
fn r_entry() {
    panic!();
}
