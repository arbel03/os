// Example: https://github.com/gz/rust-x86/blob/master/src/bits32/irq.rs
use core::fmt;

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

#[repr(C, packed)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

impl fmt::Display for ExceptionStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExceptionStackFrame {{
    Instruction Pointer: {:#x}
    Code Segment: {:#x}
    CPU Flags: {:#x}
    Stack Pointer: {:#x}
    Stack Segment: {:#x}
}}",
            &self.instruction_pointer, 
            &self.code_segment, 
            &self.cpu_flags, 
            &self.stack_pointer, 
            &self.stack_segment)
    }
}

impl IdtEntry {
    pub const MISSING: IdtEntry = IdtEntry {
        base_low: 0,
        selector: 0,
        zero: 0,
        flags: 0,
        base_high: 0,
    };

    pub fn new(isr: u32) -> Self {
        let base_low = (isr & 0xFFFF) as u16;
        let selector: u16 = 0x08; // My code segment
        let zero: u8 = 0;
        let flags: u8 = Flags::Present as u8 | Flags::DPL3 as u8 | Flags::GateInterrupt32 as u8;
        let base_high: u16 = ((isr >> 16) & 0xFFFF) as u16;

        Self {
            base_low: base_low,
            selector: selector,
            zero: zero,
            flags: flags,
            base_high: base_high,
        }
    }
}

#[repr(C, packed)]
pub struct Exceptions {
    pub divide_by_zero: IdtEntry,
    pub debug: IdtEntry,
    pub non_maskable_interrupt: IdtEntry,
    pub breakpoint: IdtEntry,
    pub overflow: IdtEntry,
    pub bound_range_exceeded: IdtEntry,
    pub invalid_opcode: IdtEntry,
    pub device_not_available: IdtEntry,
    pub double_fault: IdtEntry,
    pub invalid_tss: IdtEntry,
    pub segment_not_present: IdtEntry,
    pub stack_segment_fault: IdtEntry,
    pub general_protection_fault: IdtEntry,
    pub page_fault: IdtEntry,
    reserved1: IdtEntry,
    pub x87_floating_point: IdtEntry,
    pub alignment_check: IdtEntry,
    pub machine_check: IdtEntry,
    pub simd_floating_point: IdtEntry,
    pub virtualization: IdtEntry,
    pub reserved2: [IdtEntry; 9],
    pub security_exception: IdtEntry,
    pub reserved3: [IdtEntry; 2]
}

impl Exceptions {
    const fn new() -> Self {
        Exceptions {
            divide_by_zero: IdtEntry::MISSING,
            debug: IdtEntry::MISSING,
            non_maskable_interrupt: IdtEntry::MISSING,
            breakpoint: IdtEntry::MISSING,
            overflow: IdtEntry::MISSING,
            bound_range_exceeded: IdtEntry::MISSING,
            invalid_opcode: IdtEntry::MISSING,
            device_not_available: IdtEntry::MISSING,
            double_fault: IdtEntry::MISSING,
            invalid_tss: IdtEntry::MISSING,
            segment_not_present: IdtEntry::MISSING,
            stack_segment_fault: IdtEntry::MISSING,
            general_protection_fault: IdtEntry::MISSING,
            page_fault: IdtEntry::MISSING,
            reserved1: IdtEntry::MISSING,
            x87_floating_point: IdtEntry::MISSING,
            alignment_check: IdtEntry::MISSING,
            machine_check: IdtEntry::MISSING,
            simd_floating_point: IdtEntry::MISSING,
            virtualization: IdtEntry::MISSING,
            reserved2: [IdtEntry::MISSING; 9],
            security_exception: IdtEntry::MISSING,
            reserved3: [IdtEntry::MISSING; 2],
        }
    }
}

#[repr(C, packed)]
pub struct Idt {
    pub exceptions: Exceptions,
    hardware_interrupts: [IdtEntry; 32],
    pub interrupts: [IdtEntry; 191],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            exceptions: Exceptions::new(),
            hardware_interrupts: [IdtEntry::MISSING; 32],
            interrupts: [IdtEntry::MISSING; 191],
        }
    }

    pub fn set_hardware_interrupt(&mut self, index: usize, idt: IdtEntry) {
        if index < self.hardware_interrupts.len() {
            self.hardware_interrupts[index] = idt;
        }
    }

    pub unsafe fn load(&self) {
        use dtables::{ TableDescriptor, lidt };
        use core::slice;
        let idt_slice = slice::from_raw_parts(self as *const _ as *const IdtEntry, 256);
        let idtr = TableDescriptor::new(idt_slice);
        lidt(&idtr);
    }
}