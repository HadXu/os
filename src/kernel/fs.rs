use crate::{kernel, println};
use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

const MAGIC: &'static str = "MOROS FS";

const BITMAP_SIZE: u32 = 512 - 4; // TODO: Bitmap should use the full block
const MAX_BLOCKS: u32 = 2 * 2048;

const DISK_OFFSET: u32 = (1 << 20) / 512;
const SUPERBLOCK_ADDR: u32 = DISK_OFFSET;
const BITMAP_ADDR_OFFSET: u32 = DISK_OFFSET + 2;
const DATA_ADDR_OFFSET: u32 = BITMAP_ADDR_OFFSET + MAX_BLOCKS / 8;

lazy_static! {
    pub static ref BLOCK_DEVICE: Mutex<Option<BlockDevice>> = Mutex::new(None);
}
pub struct BlockDevice {
    bus: u8,
    dsk: u8,
}

impl BlockDevice {
    pub fn new(bus: u8, dsk: u8) -> Self {
        Self { bus, dsk }
    }

    pub fn read(&self, block: u32, mut buf: &mut [u8]) {
        kernel::ata::read(self.bus, self.dsk, block, &mut buf);
    }

    pub fn write(&self, block: u32, buf: &[u8]) {
        kernel::ata::write(self.bus, self.dsk, block, &buf);
    }
}

pub fn mount(bus: u8, dsk: u8) {
    let block_device = BlockDevice::new(bus, dsk);
    *BLOCK_DEVICE.lock() = Some(block_device);
}


pub fn format(bus: u8, dsk: u8) {
    // Write superblock
    let mut buf = MAGIC.as_bytes().to_vec();
    buf.resize(512, 0);
    println!("{}", buf[0]);
    let block_device = BlockDevice::new(bus, dsk);
    block_device.write(SUPERBLOCK_ADDR, &buf);
    mount(bus, dsk);
}

pub fn init() {
    for bus in 0..2 {
        for dsk in 0..2 {
            let mut buf = [0u8; 512];
            kernel::ata::read(bus, dsk, SUPERBLOCK_ADDR, &mut buf);
            if let Ok(header) = String::from_utf8(buf[0..8].to_vec()) {
                if header == MAGIC {
                    println!("MFS Superblock found in ATA {}:{}\n", bus, dsk);
                    mount(bus, dsk);
                }
            }
        }
    }
}
