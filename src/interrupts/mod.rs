mod idt;
mod syscall;
mod exceptions;

use drivers;
use self::exceptions::*;

static mut IDT: idt::Idt = idt::Idt::new();

pub fn init() {
    unsafe {
        IDT.exceptions.double_fault = idt::IdtEntry::new(double_fault as u32);
        IDT.set_hardware_interrupt(1, idt::IdtEntry::new(keyboard_irq as u32)); 
        IDT.exceptions.bound_range_exceeded = idt::IdtEntry::new(bound_range_exceeded as u32);             
        IDT.exceptions.general_protection_fault = idt::IdtEntry::new(general_protection_fault as u32);       
        IDT.set_hardware_interrupt(14, idt::IdtEntry::new(primary_ata_controller as u32));

        // Setup syscalls
        syscall::init();

        IDT.load();
        // Enable hardware interrupts
        asm!("sti");
    }
}

extern "x86-interrupt" fn primary_ata_controller(_stack_frame: &idt::ExceptionStackFrame) {
    // println!("Primary ATA controller interrupted.");
    drivers::send_eoi(true);
}

extern "x86-interrupt" fn keyboard_irq(_stack_frame: &idt::ExceptionStackFrame) {
    if let Some(c) = drivers::keyboard::getc() {
        print!("{}", c);
    }
    drivers::send_eoi(false);
}