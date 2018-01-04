use drivers::utils::*;

static PIC1_COMMAND: u16 = 0x20;
static PIC1_DATA: u16 = 0x21;
static PIC2_COMMAND: u16 = 0xA0;
static PIC2_DATA: u16 = 0xA1;

static INIT_COMMAND: u8 = 0x11;
static EOI_COMMAND: u8 = 0x20;
static MODE_8086: u8 = 0x01;

// PIC end of interrupt function
// pub unsafe fn pic_eoi(irq: u8) {
//     if(irq >= 8) {
// 		outb(PIC2_COMMAND,EOI_COMMAND);
//     }
// 	outb(PIC1_COMMAND,EOI_COMMAND);
// }

pub fn configure() {
    unsafe {
        let a1 = inb(PIC1_DATA); // mask 1

        outb(PIC1_COMMAND, INIT_COMMAND); // Start initializing pic 1
        io_wait();

        // Master PIC vector offset- 32 (the first 32 idt entries are for intel's exceptions)
        outb(PIC1_DATA, 0x20);
        io_wait();
        // Tell master PIC that there is a slave PIC at IRQ 2
        outb(PIC1_DATA, 4);
        io_wait();
        // Set mode
        outb(PIC1_DATA, MODE_8086);
        io_wait();

        outb(PIC1_DATA, a1); // restore mask
    }
}