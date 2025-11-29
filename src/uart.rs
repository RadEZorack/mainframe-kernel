use core::ptr::{read_volatile, write_volatile};

// PL011 UART @ 0x0900_0000 on QEMU virt
const UART0_BASE: u64 = 0x0900_0000;
const UARTDR: u64 = 0x00;
const UARTFR: u64 = 0x18;
const UARTIBRD: u64 = 0x24;
const UARTFBRD: u64 = 0x28;
const UARTLCR_H: u64 = 0x2C;
const UARTCR: u64 = 0x30;
const UARTICR: u64 = 0x44;

fn reg(offset: u64) -> *mut u32 {
    (UART0_BASE + offset) as *mut u32
}

pub fn uart_init() {
    unsafe {
        // Disable UART & clear interrupts before programming.
        write_volatile(reg(UARTCR), 0);
        write_volatile(reg(UARTICR), 0x7FF);

        // 24MHz clock / (16 * 115200) ≈ 13.0x → integer 13, fractional 0.
        write_volatile(reg(UARTIBRD), 13);
        write_volatile(reg(UARTFBRD), 0);

        const LCRH_WLEN_8: u32 = 0b11 << 5;
        const LCRH_FEN: u32 = 1 << 4;
        write_volatile(reg(UARTLCR_H), LCRH_WLEN_8 | LCRH_FEN);

        const CR_UARTEN: u32 = 1 << 0;
        const CR_TXE: u32 = 1 << 8;
        const CR_RXE: u32 = 1 << 9;
        write_volatile(reg(UARTCR), CR_UARTEN | CR_TXE | CR_RXE);
    }
}

pub fn uart_putc(c: u8) {
    unsafe {
        // wait until TX FIFO not full (bit 5 = TXFF)
        while read_volatile(reg(UARTFR)) & (1 << 5) != 0 {}

        write_volatile(reg(UARTDR), c as u32);
    }
}

pub fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}
