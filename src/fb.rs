/// Hardcoded RAMFB base on QEMU macOS ARM build.
///
/// QEMU maps ramfb memory at 0x4000_0000 for `virt` machines.
const RAMFB_BASE: usize = 0x4000_0000;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub fn ramfb_blue_screen() {
    let fb_ptr = RAMFB_BASE as *mut u32;
    let total_pixels = WIDTH * HEIGHT;

    unsafe {
        for i in 0..total_pixels {
            core::ptr::write_volatile(fb_ptr.add(i), 0x000000FF);
        }
    }
}
