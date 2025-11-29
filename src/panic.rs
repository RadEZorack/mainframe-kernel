use crate::uart::uart_puts;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts("KERNEL PANIC: ");
    if let Some(msg) = info.message() {
        // using core::fmt machinery is ok
    }
    uart_puts("System halted.\n");

    loop {}
}
