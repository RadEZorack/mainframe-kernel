#![allow(dead_code)]

pub const VIRTIO_GPU_MMIO_BASE: u64 = 0x0A000000;

#[repr(C)]
pub struct VirtioGpuConfig {
    pub events_read: u32,
    pub events_clear: u32,
    pub num_scanouts: u32,
    pub reserved: u32,
}
