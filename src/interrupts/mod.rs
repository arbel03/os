mod idt;
use dtables::{TableDescriptor, lidt};
use drivers::pic;

static mut IDT: [idt::IdtEntry; 256] = [idt::IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        IDT[8] = idt::IdtEntry::new(double_fault as u32);
        IDT[32] = idt::IdtEntry::new(keyboard_irq as u32);

        // Configure hardware interrupts
        pic::configure();
        asm!("sti");

        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);
    }
}

extern "x86-interrupt" fn keyboard_irq(stack_frame: &idt::ExceptionStackFrame) {
    println!("\nKeyboard pressed!\n");
}

extern "x86-interrupt" fn double_fault(error_code: u8, stack_frame: &idt::ExceptionStackFrame) {
    println!("Exception! Double Fault.");
    println!("{}", stack_frame);
    loop {};
}
