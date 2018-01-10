mod utils;
mod pic;
pub mod keyboard;
use self::pic::PIC;

static PIC1: PIC = PIC::new(0x20, 0x21);
static PIC2: PIC = PIC::new(0xA0, 0xA1);

pub fn configure() {
    // Initializing master PIC as master
    PIC1.init(0x20, true);
    PIC2.init(0x28, false);

    PIC1.disable_irq(0); // Disable timer for now
    PIC1.enable_irq(1); // Keyboard
    PIC1.enable_irq(2); // Slave PIC
}

// PIC end of interrupt function
pub fn send_eoi(slave_irq: bool) {
    if slave_irq {
		PIC2.send_eoi(); // send to slave
    }
    PIC1.send_eoi(); // send to master- always required
}