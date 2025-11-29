#![no_std]
#![no_main]

mod panic;
mod uart;
mod print;
mod arch;

use crate::uart::{uart_init, uart_puts};

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_init();
    uart_puts("\n\n=== MAINFRAME KERNEL BOOTED ===\n");
    uart_puts("Hello from Rust kernel_main!\n");

    loop {}
}
