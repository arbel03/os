mod idt;
use dtables::{TableDescriptor, lidt};
use drivers;

static mut IDT: [idt::IdtEntry; 256] = [idt::IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        // Enable hardware interrupts
        asm!("sti");

        IDT[8] = idt::IdtEntry::new(double_fault as u32);
        IDT[33] = idt::IdtEntry::new(keyboard_irq as u32);       

        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);
    }
}

extern "x86-interrupt" fn keyboard_irq(stack_frame: &idt::ExceptionStackFrame) {
    if let Some(c) = drivers::keyboard::getc() {
        print!("{}", c);
    }
    drivers::send_eoi(false);
}

extern "x86-interrupt" fn double_fault(error_code: u8, stack_frame: &idt::ExceptionStackFrame) {
    println!("Exception! Double Fault.(code {})", error_code);
    println!("{}", stack_frame);
    loop {};
}
