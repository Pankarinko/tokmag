const BASE: *const u8 = 0x10000000 as *const u8;
const RBR: *const u8 = BASE;
const LSR_OFFSET: usize = 5;
const LCR_OFFSET: usize = 3;
const FCR_OFFSET: usize = 2;
const THR_OFFSET: usize = 0;

/* I set the comm parameters to 8 data bits, one stop bit, no parity as recommended by https://www.lammertbies.nl/comm/info/serial-uart
    I turn off FIFO
    We'll see if this makes any difference
*/
fn init_uart() {
    unsafe {
        let lcr: *mut u8 = (BASE as *mut u8).add(LCR_OFFSET);
        *lcr = 0x43;
        let fcr: *mut u8 = (BASE as *mut u8).add(FCR_OFFSET);
        *fcr = 0x0;
    }
}

fn write_byte(c: u8) {
    let rbr: *mut u8 = RBR as *mut u8;
    unsafe {
        while *(BASE as *mut u8).add(LCR_OFFSET) & 0x10 == 0x10 {};
        *rbr = c };
}
