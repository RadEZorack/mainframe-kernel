#![no_std]
#![no_main]

use crate::uart::uart_puts;   // <-- ADD THIS LINE

pub mod panic;
pub mod uart;
pub mod arch;
pub mod virtio_gpu;
pub mod print;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_puts("BOOTING...\n");
    uart_puts("About to init GPU...\n");

    virtio_gpu::gpu_init();

    uart_puts("GPU init returned\n");

    loop {}
}
