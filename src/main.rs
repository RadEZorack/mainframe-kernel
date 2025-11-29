#![no_std]
#![no_main]

mod arch;      // <-- this brings in the aarch64 module
mod uart;
mod panic;
mod print;
mod virtio_gpu;

use uart::uart_puts;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart::init();
    uart_puts("KERNEL MAIN REACHED\n");
    loop {}
}
