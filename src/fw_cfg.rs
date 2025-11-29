use core::convert::TryInto;
use core::ptr::{read_volatile, write_volatile};
use core::slice;
use core::sync::atomic::{compiler_fence, Ordering};

const FW_CFG_BASE: u64 = 0x0090_2000;
const FW_CFG_CTRL: u64 = FW_CFG_BASE + 0x08;
const FW_CFG_DMA: u64 = FW_CFG_BASE + 0x10;

const FW_CFG_FILE_DIR: u16 = 0x0019;

const FW_CFG_DMA_CTL_ERROR: u32 = 0x01;
const FW_CFG_DMA_CTL_READ: u32 = 0x02;
const FW_CFG_DMA_CTL_SKIP: u32 = 0x04;
const FW_CFG_DMA_CTL_SELECT: u32 = 0x08;
const FW_CFG_DMA_CTL_WRITE: u32 = 0x10;

const FW_CFG_FILE_ENTRY_SIZE: usize = 64;
const RAMFB_FILE_NAME: &str = "etc/ramfb";
const DRM_FORMAT_XRGB8888: u32 = u32::from_le_bytes(*b"XR24");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// fw_cfg or the ramfb file is not present in this machine configuration.
    NotAvailable,
    /// fw_cfg reported a DMA error when transferring data.
    DmaFailed,
}

type Result<T> = core::result::Result<T, Error>;

#[repr(C, align(16))]
struct FwCfgDmaAccess {
    control: u32,
    length: u32,
    address: u64,
}

#[repr(C)]
struct RamfbCfg {
    addr: u64,
    fourcc: u32,
    flags: u32,
    width: u32,
    height: u32,
    stride: u32,
}

impl RamfbCfg {
    fn new(addr: u64, width: u32, height: u32, stride: u32) -> Self {
        Self {
            addr: addr.to_be(),
            fourcc: DRM_FORMAT_XRGB8888.to_be(),
            flags: 0,
            width: width.to_be(),
            height: height.to_be(),
            stride: stride.to_be(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const RamfbCfg as *const u8,
                core::mem::size_of::<RamfbCfg>(),
            )
        }
    }
}

struct FileEntry {
    select: u16,
    _size: u32,
}

fn fw_cfg_select(key: u16) {
    unsafe {
        write_volatile(FW_CFG_CTRL as *mut u16, key.to_be());
    }
}

fn fw_cfg_dma_transfer(address: u64, length: u32, control: u32) -> Result<()> {
    if length == 0 {
        return Ok(());
    }

    let mut desc = FwCfgDmaAccess {
        control: control.to_be(),
        length: length.to_be(),
        address: address.to_be(),
    };

    let desc_ptr = &mut desc as *mut _ as u64;
    compiler_fence(Ordering::SeqCst);

    unsafe {
        write_volatile(FW_CFG_DMA as *mut u32, (desc_ptr >> 32) as u32);
        write_volatile((FW_CFG_DMA as *mut u32).add(1), desc_ptr as u32);

        loop {
            let ctrl = u32::from_be(read_volatile(&desc.control));
            if (ctrl & !FW_CFG_DMA_CTL_ERROR) == 0 {
                break;
            }
        }
    }

    let status = u32::from_be(desc.control);
    if (status & FW_CFG_DMA_CTL_ERROR) != 0 {
        return Err(Error::DmaFailed);
    }

    Ok(())
}

fn fw_cfg_read_bytes(key: u16, offset: usize, buf: &mut [u8]) -> Result<()> {
    fw_cfg_select(key);
    if offset > 0 {
        fw_cfg_dma_transfer(0, offset as u32, FW_CFG_DMA_CTL_SKIP)?;
    }
    if !buf.is_empty() {
        fw_cfg_dma_transfer(buf.as_mut_ptr() as u64, buf.len() as u32, FW_CFG_DMA_CTL_READ)?;
    }
    Ok(())
}

fn fw_cfg_write_bytes(key: u16, buf: &[u8]) -> Result<()> {
    if buf.is_empty() {
        return Ok(());
    }

    let control = ((key as u32) << 16) | FW_CFG_DMA_CTL_SELECT | FW_CFG_DMA_CTL_WRITE;
    fw_cfg_dma_transfer(buf.as_ptr() as u64, buf.len() as u32, control)
}

fn read_directory_entry(index: usize) -> Result<FileEntry> {
    let offset = 4 + index * FW_CFG_FILE_ENTRY_SIZE;
    let mut raw = [0u8; FW_CFG_FILE_ENTRY_SIZE];
    fw_cfg_read_bytes(FW_CFG_FILE_DIR, offset, &mut raw)?;

    let size = u32::from_be_bytes(raw[0..4].try_into().unwrap());
    let select = u16::from_be_bytes(raw[4..6].try_into().unwrap());
    let name_len = raw[8..]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(FW_CFG_FILE_ENTRY_SIZE - 8);
    let name_bytes = &raw[8..8 + name_len];
    let Ok(name) = core::str::from_utf8(name_bytes) else {
        return Err(Error::NotAvailable);
    };

    if name == RAMFB_FILE_NAME {
        Ok(FileEntry { select, _size: size })
    } else {
        Err(Error::NotAvailable)
    }
}

fn find_ramfb_entry() -> Result<FileEntry> {
    let mut count_buf = [0u8; 4];
    fw_cfg_read_bytes(FW_CFG_FILE_DIR, 0, &mut count_buf)?;
    let entries = u32::from_be_bytes(count_buf) as usize;

    for idx in 0..entries {
        if let Ok(entry) = read_directory_entry(idx) {
            return Ok(entry);
        }
    }

    Err(Error::NotAvailable)
}

pub fn configure_ramfb(addr: u64, width: u32, height: u32, stride: u32) -> Result<()> {
    let entry = find_ramfb_entry()?;
    let cfg = RamfbCfg::new(addr, width, height, stride);
    fw_cfg_write_bytes(entry.select, cfg.as_bytes())
}

