#![no_std]

pub mod panic;
pub mod uart;
pub mod arch;

use crate::uart::uart_puts;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_puts("Mainframe ARM64 Kernel Booted Successfully!\n");

    loop {}
}
