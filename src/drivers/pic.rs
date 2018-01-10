use drivers::utils::*;

pub struct PIC {
    command_port: u16,
    data_port: u16
}

impl PIC {
    pub const fn new(command_port: u16, data_port: u16) -> PIC {
        PIC { command_port: command_port, data_port: data_port }
    }

    // This function enables an irq of a certain pic
    pub fn enable_irq(&self, irq_line: u8) {
        let irq_line = irq_line % 8;
        unsafe {
            let value = inb(self.data_port) & !(1 << irq_line);
            outb(self.data_port, value);
        }
    }

    // This function disables an irq of a certain pic
    pub fn disable_irq(&self, irq_line: u8) {
        let irq_line = irq_line % 8;
        unsafe {
            let value = inb(self.data_port) | (1 << irq_line);
            outb(self.data_port, value);
        }
    }

    // Sending an end of interrupt command, needed to continue receiving more irqs.
    pub fn send_eoi(&self) {
        const EOI_COMMAND: u8 = 0x20;
        unsafe {
            outb(self.command_port, EOI_COMMAND);
        }
    }

    pub fn init(&self, offset: u8, is_master: bool) {
        const INIT_COMMAND: u8 = 0x11;
        const MODE_8086: u8 = 0x01;

        unsafe {
            // Save mask
            let mask = inb(self.data_port);
            // Start initializing pic 1
            outb(self.command_port, INIT_COMMAND);
            io_wait();

            // Master PIC vector offset- 32 (the first 32 idt entries are for intel's exceptions)
            outb(self.data_port, offset);
            io_wait();
            // Tell master PIC that there is a slave PIC at IRQ 2
            outb(self.data_port, if is_master { 4 } else { 2 });
            io_wait();
            // Set mode
            outb(self.data_port, MODE_8086);
            io_wait();
            // Restore mask
            outb(self.data_port, mask); 
        }
    }
}