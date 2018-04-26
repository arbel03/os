pub mod ata;
pub mod keyboard;
pub mod cursor;
mod utils;
mod pic;
use self::pic::Pic;

pub fn configure() {
    // Initializing master PIC as master
    Pic::MASTER.init(0x20, true);
    Pic::SLAVE.init(0x28, false);

    Pic::MASTER.disable_irq(0); // Disable timer for now
    Pic::MASTER.disable_irq(1); // Keyboard
    Pic::MASTER.disable_irq(2); // Slave PIC
}

// PIC end of interrupt function
pub fn send_eoi(slave_irq: bool) {
    Pic::MASTER.send_eoi(); // send to master- always required
    if slave_irq {
		Pic::SLAVE.send_eoi(); // send to slave
    }
}