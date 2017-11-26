// Example: https://github.com/gz/rust-x86/blob/master/src/bits32/irq.rs

#[allow(dead_code)]
#[repr(u8)]
enum Flags {
    Present = 0b10000000,
    DPL0 = 0b00000000,
    DPL1 = 0b00100000,
    DPL2 = 0b01000000,
    DPL3 = 0b01100000,
    Storage = 0b00010000,
    GateTask32 = 0x5,
    GateInterrupt16 = 0x6,
    GateTrap16 = 0x7,
    GateInterrupt32 = 0xE,
    GateTrap32 = 0xF,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct IdtEntry {
    base_low: u16, // Lower address of ISR
    selector: u16,
    zero: u8,
    flags: u8,
    base_high: u16 // Higher address of the ISR
}

// TODO: Implement generic new functions to link ISR
impl IdtEntry {
    pub const MISSING: IdtEntry = IdtEntry {
        base_low: 0,
        selector: 0,
        zero: 0,
        flags: 0,
        base_high: 0,
    };

    pub fn new(isr: u32) -> IdtEntry {
        let base_low = (isr & 0xFFFF) as u16;
        let selector: u16 = 0x08; // My code segment
        let zero: u8 = 0;
        let flags: u8 = Flags::Present as u8 | Flags::DPL0 as u8 | Flags::GateInterrupt32 as u8;
        let base_high: u16 = ((isr >> 16) & 0xFFFF) as u16;

        IdtEntry {
            base_low: base_low,
            selector: selector,
            zero: zero,
            flags: flags,
            base_high: base_high,
        }
    }
}

