use core::str;

#[repr(u8)]
#[allow(dead_code)]
pub enum Drive {
    Master = 0xA0,
    Slave = 0xB0,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DeviceType {
    Ata,
    Atapi,
    Sata,
    Satapi,
}

// Have Ata implement this
pub trait Disk {
    unsafe fn read(&self, block: u64, buffer: &mut [u8]) -> Result<u8, &str>;
    unsafe fn write_at(&self, block: u64, buffer: &[u8]) -> Result<u8, &str>;
}