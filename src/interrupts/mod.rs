mod idt;
use dtables::{TableDescriptor, lidt};
use drivers::pic;
use drivers::keyboard;

static mut IDT: [idt::IdtEntry; 256] = [idt::IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        // Configure hardware interrupts
        pic::configure();
        asm!("sti");

        IDT[8] = idt::IdtEntry::new(double_fault as u32);
        IDT[33] = idt::IdtEntry::new(keyboard_irq as u32);       

        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);
    }
}

extern "x86-interrupt" fn keyboard_irq(stack_frame: &idt::ExceptionStackFrame) {
    //println!("Key pressed!, code: {}", keyboard::get_scancode());
    if let Some(c) = keyboard::getc() {
        print!("{}", c);
    }
    pic::send_eoi(false);
}

extern "x86-interrupt" fn double_fault(error_code: u8, stack_frame: &idt::ExceptionStackFrame) {
    println!("Exception! Double Fault.(code {})", error_code);
    println!("{}", stack_frame);
    loop {};
}
