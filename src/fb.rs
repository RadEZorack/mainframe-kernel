/// Paint the default RAM framebuffer bright blue so we know the kernel ran.
pub fn ramfb_blue_screen() {
    // RAMFB default base on QEMU virt board when `-device ramfb`.
    const FB_BASE: *mut u32 = 0x40000 as *mut u32;
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    unsafe {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = FB_BASE.add(y * WIDTH + x);
                *pixel = 0x0000_00FF; // BGRA little-endian: solid blue
            }
        }
    }
}
