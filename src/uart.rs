const UART0_BASE: u64 = 0x09000000;

fn uart_reg(offset: u64) -> *mut u8 {
    (UART0_BASE + offset) as *mut u8
}

pub fn uart_putc(c: u8) {
    unsafe {
        // Wait until TX FIFO not full
        while core::ptr::read_volatile(uart_reg(0x18)) & (1 << 5) != 0 {}

        core::ptr::write_volatile(uart_reg(0x00), c);
    }
}

pub fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}
