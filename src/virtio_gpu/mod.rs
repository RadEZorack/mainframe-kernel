use crate::uart::uart_puts;
use core::ptr::{read_volatile, write_volatile};

const VIRTIO_GPU_F_VIRGL: u32 = 0;
const VIRTIO_MMIO_BASE: u64 = 0x0A000000;
const MMIO_SIZE: usize = 0x200;

fn mmio(offset: usize) -> *mut u32 {
    (VIRTIO_MMIO_BASE + offset as u64) as *mut u32
}

pub fn gpu_init() {
    uart_puts("GPU: init\n");

    unsafe {
        // Reset
        write_volatile(mmio(0x014), 0);

        // Status: ACK + DRIVER
        write_volatile(mmio(0x070), 1);
        write_volatile(mmio(0x070), 2);
    }

    gpu_blue_screen();
}

fn gpu_blue_screen() {
    uart_puts("GPU: blue screen\n");

    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    const FB_SIZE: usize = WIDTH * HEIGHT * 4;

    static mut FRAMEBUFFER: [u32; (640*480)] = [0; 640*480];

    unsafe {
        // Fill with blue
        for px in FRAMEBUFFER.iter_mut() {
            *px = 0x0000FF;   // Blue (RGB)
        }

        // 1️⃣ Create resource
        write_volatile(mmio(0x100), 0); // type = 0 = 2D_RESOURCE_CREATE
        write_volatile(mmio(0x104), 1); // resource_id = 1
        write_volatile(mmio(0x108), WIDTH as u32);
        write_volatile(mmio(0x10C), HEIGHT as u32);
        write_volatile(mmio(0x110), 1); // format RGBA
        write_volatile(mmio(0x120), 1); // notify
    }

    uart_puts("GPU: complete\n");
}
