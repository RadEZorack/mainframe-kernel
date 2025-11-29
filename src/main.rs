#![no_std]
#![no_main]

use crate::uart::uart_puts;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_puts("🌟 J’SOS ARM64 Kernel Booted Successfully!\n");

    loop {}
}
