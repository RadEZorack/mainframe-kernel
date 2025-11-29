#![no_std]
#![no_main]

extern crate mainframe_kernel;

use mainframe_kernel::kernel_main;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}
