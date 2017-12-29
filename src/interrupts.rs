use dtables::idt::{IdtEntry, ExceptionStackFrame};
use dtables::{TableDescriptor, lidt};

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);

        IDT[3] = IdtEntry::new(breakpoint_handler as u32);
        
        // TODO: Fix triple fault
        // http://wiki.osdev.org/I_Can't_Get_Interrupts_Working
        asm!("int 3" :::: "intel");
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    println!("Exception!: Breakpoint");
    println!("Info: {:?}", stack_frame);
}

