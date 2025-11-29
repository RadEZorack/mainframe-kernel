use crate::uart::uart_puts;
use core::fmt::{Write};
use core::panic::PanicInfo;

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_puts(s);
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts("KERNEL PANIC!\n");

    if let Some(loc) = info.location() {
        uart_puts("Location: ");
        uart_puts(loc.file());
        uart_puts(":");

        let mut w = UartWriter;
        let _ = write!(w, "{}", loc.line());
        uart_puts("\n");
    }

    uart_puts("Message: ");
    let msg = info.message();
    let mut w = UartWriter;

    if let Some(s) = msg.as_str() {
        uart_puts(s);
    } else {
        let _ = write!(w, "{:?}", msg);
    }

    uart_puts("\nSystem halted.\n");

    loop {}
}
