#![no_std]
#![no_main]
mod panic_handler;

#[unsafe(no_mangle)]
fn r_entry() {
    panic!();
}
