use dtables::idt::*;
use dtables::*;

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        IDT[8] = IdtEntry::new(double_fault_handler);
        IDT[3] = IdtEntry::new(breakpoint_handler);
        
        let idt_pointer = TablePointer::new(&IDT);
        lidt(&idt_pointer);
    }
}


extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("Exception!: Double Fault");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("Exception!: Breakpoint");
}