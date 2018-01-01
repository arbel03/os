use dtables::{TableDescriptor, lidt};
mod idt;
use self::idt::*;

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);

        IDT[3] = IdtEntry::new(breakpoint_handler as u32);
        
        // Sending an example interrupt
        asm!("int 3" :::: "intel");
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nBreakpoint Exception! Info: \n{}\n", stack_frame);
}
