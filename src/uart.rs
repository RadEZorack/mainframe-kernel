use core::ptr::{read_volatile, write_volatile};

const UART0_BASE: u64 = 0x09000000;

const UARTDR: u64 = 0x00;
const UARTFR: u64 = 0x18;
const UARTIBRD: u64 = 0x24;
const UARTFBRD: u64 = 0x28;
const UARTLCR_H: u64 = 0x2C;
const UARTCR: u64 = 0x30;
const UARTIMSC: u64 = 0x38;
const UARTICR: u64 = 0x44;

fn reg(offset: u64) -> *mut u32 {
    (UART0_BASE + offset) as *mut u32
}

unsafe fn write_reg(offset: u64, value: u32) {
    write_volatile(reg(offset), value);
}

unsafe fn read_reg(offset: u64) -> u32 {
    read_volatile(reg(offset))
}

pub fn init() {
    unsafe {
        // Make sure the UART is disabled before we touch the control regs.
        write_reg(UARTCR, 0);

        // Clear any stale interrupts and mask them off.
        write_reg(UARTICR, 0x7FF);
        write_reg(UARTIMSC, 0);

        // Program the baud rate divisors for 115200 using the 24 MHz clock.
        write_reg(UARTIBRD, 13);
        write_reg(UARTFBRD, 1);

        // 8-bit words, FIFO enabled, no parity, one stop bit.
        write_reg(UARTLCR_H, (0b11 << 5) | (1 << 4));

        // Enable UART, RX, and TX.
        write_reg(UARTCR, (1 << 9) | (1 << 8) | (1 << 0));
    }
}

fn write_byte(byte: u8) {
    unsafe {
        while read_reg(UARTFR) & (1 << 5) != 0 {}
        write_reg(UARTDR, byte as u32);
    }
}

pub fn uart_putc(c: u8) {
    if c == b'\n' {
        write_byte(b'\r');
    }
    write_byte(c);
}

pub fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}
