use crate::uart::uart_puts;

pub fn println(msg: &str) {
    uart_puts(msg);
    uart_puts("\n");
}
