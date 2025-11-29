// Simple MMIO read/write helpers

use core::ptr::{read_volatile, write_volatile};

#[inline(always)]
pub unsafe fn mmio_read32(addr: usize) -> u32 {
    read_volatile(addr as *const u32)
}

#[inline(always)]
pub unsafe fn mmio_write32(addr: usize, value: u32) {
    write_volatile(addr as *mut u32, value);
}

#[inline(always)]
pub unsafe fn mmio_write64(addr: usize, value: u64) {
    write_volatile(addr as *mut u64, value);
}
