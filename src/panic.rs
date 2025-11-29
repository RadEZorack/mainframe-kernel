use core::panic::PanicInfo;
use crate::uart::uart_puts;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts("KERNEL PANIC: ");
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        uart_puts(msg);
    }
    uart_puts("\n");
    loop {}
}
