// Primary ATA bus- control ports(0x1F0-0x1F7), status(0x3F6)
//      IRQ14
// Secondary ATA bus- control ports(0x170-0x177), status(0x376)
//      IRQ15
// Each buss has 2 devices- master and slave
use drivers::utils::*;
use filesystem::disk::{ DeviceType, Disk };
use core::slice;

pub struct Ata {
    control_ports: PortRange,
    status_port: u16,
}

#[allow(dead_code)]
pub enum AtaIdentifyResponse {
    ValidDevice,
    InvalidDevice(DeviceType),
    DoesntExist,
}

#[derive(Copy, Clone)]
#[repr(u16)]
#[allow(dead_code)]
enum RegisterType {
    ErrorInformation = 1,
    SectorCount = 2,
    LbaLow = 3,
    LbaMid = 4,
    LbaHigh = 5,
    Drive = 6,
    Command = 7,
    Status,
}

impl Ata {
    pub const PRIMARY: Ata = Ata::new(PortRange::new(0x1F0, 0x1F7), 0x3F6);

    pub const fn new(control_ports: PortRange, status_port: u16) -> Self {
        Ata {
            control_ports: control_ports,
            status_port: status_port,
        }
    }

    unsafe fn write_register(&self, register: RegisterType, value: u8) {
        outb(self.get_port(register), value)
    }

    fn get_port(&self, register: RegisterType) -> u16 {
        match register {
            RegisterType::Status => self.status_port,
            _ => self.control_ports.get(register as u16),
        }
    }

    unsafe fn read_register(&self, register: RegisterType) -> u8 {
        inb(self.get_port(register))
    }

    // Reading a single value from the data port
    unsafe fn read_data(&self) -> u16 {
         inw(self.control_ports.get(0))
    }

    unsafe fn poll<F>(&self, register: RegisterType, condition: F) -> u8 
        where F: Fn(u8) -> bool {
        
        let mut reg_value: u8;
        loop {
            reg_value = self.read_register(register);
            if condition(reg_value) {
                return reg_value;
            }
        }
    }
}

impl Disk for Ata {
    unsafe fn read(&self, block: u64, buffer: &mut [u8]) -> Result<u8, &str> {
        // Transform buffer into byte array
        // let new_len = buffer.len() * size_of::<T>() / size_of::<u8>();
        // let buffer = slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, new_len);

        if buffer.len() % 512 != 0 {
            return Err("Size of buffer and requested read amount doesn't match.");
        }
        if buffer.len() / 512 > 127 {
            return Err("Can only read 127 sectors at a time in LBA28 mode.");
        }

        let sector_count = (buffer.len()/512) as u8;
        let mut command: u8 = 0xE0;
        command |= ((block >> 24) & 0x0F) as u8;
        command |= 0x40; // bit 6 enabled for 28 bit LBA mode.
        
        self.write_register(RegisterType::Drive, command);
        self.write_register(RegisterType::SectorCount, sector_count);
        self.write_register(RegisterType::LbaLow, block as u8);
        self.write_register(RegisterType::LbaMid, (block >> 8) as u8);
        self.write_register(RegisterType::LbaHigh, (block >> 16) as u8);
        self.write_register(RegisterType::Command, 0x20); // READ SECTORS command
        for sector in 0..sector_count {
            // poll until (!Bussy && DataRequestReady) or Error or DriveFault
            let status = self.poll(RegisterType::Status, |x| (x & 0x80 == 0 && x & 8 != 0) || x & 1 != 0 || x & 0x20 != 0);

            if status & 1 != 0 {
                // Return amount of read sectors.
                return Ok(sector);
            } else if status & 0x20 != 0 {
                return Err("Drive Fault occured");
            }

            // Read data to buffer
            let buff = slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u16, buffer.len()/2);
            for i in 0..buff.len() {
                buff[i+(sector as usize*256)] = self.read_data();
            }

            // Give the drive a 400ns delay to reset its DRQ bit
            for _ in 0..4 {
                self.read_register(RegisterType::Status);
            }
        }
        // Return the amount of sectors read.
        Ok(sector_count)
    }

    unsafe fn write_at(&self, block: u64, buffer: &[u8]) -> Result<u8, &str> {
        unimplemented!();
    }
}
