pub struct PortRange {
    start: u16,
    end: u16,
}

impl PortRange {
    pub const fn new(start: u16, end: u16) -> Self {
        PortRange { start: start, end: end }
    }

    pub fn get(&self, index: u16) -> u16 {
        if self.end-self.start < index {
            panic!("Port out of range.");
        }
        self.start + index
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outb(port: u16, value: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    return ret;
}

#[allow(dead_code)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw %ax, %dx" :: "{dx}"(port), "{al}"(val));
}

pub unsafe fn io_wait() {
    asm!("jmp 1f;1:jmp 2f;2:" :::: "volatile");
}