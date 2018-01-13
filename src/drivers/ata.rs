// Primary ATA bus- control ports(0x1F0-0x1F7), status(0x3F6)
//      IRQ14
// Secondary ATA bus- control ports(0x170-0x177), status(0x376)
//      IRQ15
// Each buss has 2 devices- master and slave
use drivers::utils::*;

pub struct Ata {
    control_ports: PortRange,
    status_port: u16,
}

#[repr(u8)]
pub enum Drive {
    Master = 0xA0,
    Slave = 0xB0,
}

pub enum DeviceType {
    Ata,
    Atapi,
    Sata,
    Satapi,
}

pub struct IdentifyInformation([u16;256]);

pub enum AtaIdentifyResponse {
    ValidDevice(IdentifyInformation),
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

    fn select_drive(&self, drive: Drive) {
        self.write_register(RegisterType::Drive, drive as u8);
    }

    fn write_register(&self, register: RegisterType, value: u8) {
        let port = match register {
            RegisterType::Status => self.status_port,
            _ => self.control_ports.get(register as u16),
        };
        unsafe {
            outb(port, value);
        }
    }

    fn read_register(&self, register: RegisterType) -> u8 {
        let port = match register {
            RegisterType::Status => self.status_port,
            _ => self.control_ports.get(register as u16),
        };
        unsafe { 
            inb(port) 
        }
    }

    // Reading a single value from the data port
    fn read_data(&self) -> u16 {
        unsafe { inw(self.control_ports.get(0)) }
    }

    fn poll<F>(&self, register: RegisterType, condition: F) -> u8 
        where F: Fn(u8) -> bool {
        
        let mut reg_value: u8;
        loop {
            reg_value = self.read_register(register);
            if condition(reg_value) {
                return reg_value;
            }
        }
    }

    fn get_device_type(mid: u8, high: u8) -> DeviceType {
        if mid == 0 && high == 0 {
           return DeviceType::Ata;
        }
        // Return atapi for now
        DeviceType::Atapi
    }

    // http://wiki.osdev.org/ATA_PIO_Mode#IDENTIFY_command
    pub fn identify(&self, drive: Drive) -> AtaIdentifyResponse {
        self.select_drive(drive);
        self.write_register(RegisterType::SectorCount, 0);
        self.write_register(RegisterType::LbaLow, 0);
        self.write_register(RegisterType::LbaMid, 0);
        self.write_register(RegisterType::LbaHigh, 0);

        // Send identify command to command register
        self.write_register(RegisterType::Command, 0xEC);
        let status = self.read_register(RegisterType::Status);
        // If value is 0, drive does not exist.
        if status == 0 {
            return AtaIdentifyResponse::DoesntExist;
        }
        // Polling until bit 7 of the status register clears
        let status = self.poll(RegisterType::Status, |x| (x & 0x80)==0);
        
        // Decide if the device is Ata or not.
        let lba_mid = self.read_register(RegisterType::LbaMid);
        let lba_high = self.read_register(RegisterType::LbaHigh);
        let device_type = Ata::get_device_type(lba_mid, lba_high);
        // If device is not ata, return invalid device response
        match device_type {
            Ata => (),
            _ => return AtaIdentifyResponse::InvalidDevice(device_type),
        }

        // Poll until bit 3 or bit 0 sets
        let status = self.poll(RegisterType::Status, |x| (x&8) != 0 || x&1 != 0);
        // If error is clear
        if status & 1 != 1 {
            let mut buff = vec![0u16;256];
            // Filling buffer with response
            for i in &mut buff {
                *i = self.read_data(); 
            }
            // http://wiki.osdev.org/ATA_PIO_Mode#Interesting_information_returned_by_IDENTIFY
            println!("buff[83] {:b}", buff[83]);
            println!("buff[60] {:b}{:b}", buff[61], buff[60]);
            //return AtaIdentifyResponse::ValidDevice(buff as IdentifyInformation);
            return AtaIdentifyResponse::DoesntExist;
        } else {
            panic!("Error identifying disk");
        }
    }
}
