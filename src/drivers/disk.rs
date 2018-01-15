use drivers::ata::Ata;
use vga_buffer;
use core::str;

#[repr(u8)]
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
    fn read(&mut self, block: u64, buffer: &mut [u8]) -> Result<u8, &str>;
    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<u8, &str>;
}

pub fn init() {
    // Setup PCI bus here, detect disks and set the main disks
    
}

pub fn read() {
    let mut x = [0u8;1024];
    match Ata::PRIMARY.read(2, &mut x) {
        Ok(read_amount) => 
        println!("{} sectors were read.", read_amount),
        Err(err_msg) => println!("{}", err_msg),
    }
    println!("Entry point of kernel is: {:#x}", unsafe { *((&x[0x18] as *const u8) as *const u32) });
}