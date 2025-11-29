#![allow(dead_code)]

use core::ptr::write_volatile;
use crate::print::println;

mod mmio;
use mmio::{mmio_read32, mmio_write32, mmio_write64};

// The default MMIO base for virtio-gpu on QEMU ARM virt
const VIRTIO_GPU_MMIO_BASE: usize = 0x0A000000;

// VirtIO MMIO Register Offsets (from spec)
const MMIO_MAGIC:            usize = 0x000;
const MMIO_VERSION:          usize = 0x004;
const MMIO_DEVICE_ID:        usize = 0x008;
const MMIO_VENDOR_ID:        usize = 0x00c;
const MMIO_HOST_FEATURES:    usize = 0x010;
const MMIO_GUEST_FEATURES:   usize = 0x020;
const MMIO_QUEUE_SEL:        usize = 0x030;
const MMIO_QUEUE_NUM_MAX:    usize = 0x034;
const MMIO_QUEUE_NUM:        usize = 0x038;
const MMIO_QUEUE_READY:      usize = 0x044;
const MMIO_QUEUE_NOTIFY:     usize = 0x050;
const MMIO_INTERRUPT_STATUS: usize = 0x060;
const MMIO_INTERRUPT_ACK:    usize = 0x064;
const MMIO_STATUS:           usize = 0x070;
const MMIO_QUEUE_DESC_LOW:   usize = 0x080;
const MMIO_QUEUE_DESC_HIGH:  usize = 0x084;
const MMIO_DRIVER_AREA:      usize = 0x1000;

// Status bits
const STATUS_ACKNOWLEDGE: u32 = 1;
const STATUS_DRIVER:      u32 = 2;
const STATUS_FEATURES_OK: u32 = 8;
const STATUS_DRIVER_OK:   u32 = 4;

// GPU commands
const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u32       = 0x0100;
const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32     = 0x0101;
const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0102;
const VIRTIO_GPU_CMD_SET_SCANOUT: u32            = 0x0103;
const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: u32    = 0x0105;

// Simplify: one framebuffer
const FB_WIDTH:  usize = 800;
const FB_HEIGHT: usize = 600;

// Virtual framebuffer
static mut FRAMEBUFFER: *mut u32 = core::ptr::null_mut();

pub fn gpu_init() {
    unsafe {
        println("VirtIO GPU: initializing...");

        // 1. Verify device is the right one
        let magic = mmio_read32(VIRTIO_GPU_MMIO_BASE + MMIO_MAGIC);
        let version = mmio_read32(VIRTIO_GPU_MMIO_BASE + MMIO_VERSION);
        let device = mmio_read32(VIRTIO_GPU_MMIO_BASE + MMIO_DEVICE_ID);

        if magic != 0x7472 || version != 2 || device != 16 {
            println("❌ Not a virtio-gpu device!");
            return;
        }

        // 2. Reset and set driver status
        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_STATUS, 0);
        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_STATUS, STATUS_ACKNOWLEDGE);
        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_STATUS, STATUS_ACKNOWLEDGE | STATUS_DRIVER);

        // 3. Accept features (we accept none for now)
        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_HOST_FEATURES, 0);
        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_STATUS,
            STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK);

        // 4. Allocate framebuffer
        let fb_size = FB_WIDTH * FB_HEIGHT * 4;
        FRAMEBUFFER = alloc_framebuffer(fb_size);
        fill_blue(FRAMEBUFFER, FB_WIDTH * FB_HEIGHT);

        // 5. Send commands
        println("Creating GPU resource...");
        gpu_cmd_create_2d(1, FB_WIDTH as u32, FB_HEIGHT as u32);

        println("Attaching backing...");
        gpu_cmd_attach_backing(1, FRAMEBUFFER as u64, fb_size as u32);

        println("Setting scanout...");
        gpu_cmd_set_scanout(1, FB_WIDTH as u32, FB_HEIGHT as u32);

        println("Flushing to host...");
        gpu_cmd_transfer_to_host(1, FB_WIDTH as u32, FB_HEIGHT as u32);

        mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_STATUS,
            STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK);

        println("GPU init complete — blue screen should be visible!");
    }
}

// --- Framebuffer Helpers ----------------------------------------------------

unsafe fn alloc_framebuffer(size: usize) -> *mut u32 {
    extern "C" {
        static mut _heap_start: u8;
    }

    // Raw pointer to heap start
    let ptr = core::ptr::addr_of!(_heap_start) as *mut u32;

    // Zero framebuffer memory
    let words = size / 4;
    for i in 0..words {
        core::ptr::write_volatile(ptr.add(i), 0);
    }

    ptr
}


unsafe fn fill_blue(ptr: *mut u32, pixels: usize) {
    let color = 0xFF0000FF; // Blue in BGRA
    for i in 0..pixels {
        write_volatile(ptr.add(i), color);
    }
}

// --- Minimal GPU Command Senders --------------------------------------------
// These write directly to QEMU’s command buffer (very unsafe but minimal)

unsafe fn gpu_cmd_create_2d(id: u32, w: u32, h: u32) {
    let base = VIRTIO_GPU_MMIO_BASE + MMIO_DRIVER_AREA;

    mmio_write32(base + 0, VIRTIO_GPU_CMD_RESOURCE_CREATE_2D);
    mmio_write32(base + 4, id);
    mmio_write32(base + 8, 1); // format: B8G8R8A8
    mmio_write32(base + 12, w);
    mmio_write32(base + 16, h);

    mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_QUEUE_NOTIFY, 0);
}

unsafe fn gpu_cmd_attach_backing(id: u32, addr: u64, size: u32) {
    let base = VIRTIO_GPU_MMIO_BASE + MMIO_DRIVER_AREA;

    mmio_write32(base + 0, VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING);
    mmio_write32(base + 4, id);
    mmio_write32(base + 8, 1); // 1 backing entry
    mmio_write64(base + 16, addr);
    mmio_write32(base + 24, size);

    mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_QUEUE_NOTIFY, 0);
}

unsafe fn gpu_cmd_set_scanout(id: u32, w: u32, h: u32) {
    let base = VIRTIO_GPU_MMIO_BASE + MMIO_DRIVER_AREA;

    mmio_write32(base + 0, VIRTIO_GPU_CMD_SET_SCANOUT);
    mmio_write32(base + 4, 0); // scanout 0
    mmio_write32(base + 8, id);
    mmio_write32(base + 12, 0); // x
    mmio_write32(base + 16, 0); // y
    mmio_write32(base + 20, w);
    mmio_write32(base + 24, h);

    mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_QUEUE_NOTIFY, 0);
}

unsafe fn gpu_cmd_transfer_to_host(id: u32, w: u32, h: u32) {
    let base = VIRTIO_GPU_MMIO_BASE + MMIO_DRIVER_AREA;

    mmio_write32(base + 0, VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D);
    mmio_write32(base + 4, id);
    mmio_write32(base + 8, 0); // x
    mmio_write32(base + 12, 0); // y
    mmio_write32(base + 16, w);
    mmio_write32(base + 20, h);

    mmio_write32(VIRTIO_GPU_MMIO_BASE + MMIO_QUEUE_NOTIFY, 0);
}
