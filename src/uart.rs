const BASE: *const u8 = 0x10000000 as *const u8;
const RBR: *const u8 = BASE;
const LSR_OFFSET: usize = 5;
const LCR_OFFSET: usize = 3;
const FCR_OFFSET: usize = 2;
const THR_OFFSET: usize = 0;

fn init_uart() {
    unsafe {
        let lcr: *mut u8 = (BASE as *mut u8).add(LCR_OFFSET);
        *lcr = 0x43;
    }
}

fn write_byte() {}
