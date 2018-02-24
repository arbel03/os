use interrupts::IDT;
use interrupts::idt;

pub unsafe fn init() {
    // Set handler for interrupt 64,
    // Syscall Interrupt[0x80] - (Exceptions[32d] + Hardware Interrupts[32d]) = 0x40
    IDT.interrupts[0x40] = idt::IdtEntry::new(syscall_handler as u32);
}

extern "x86-interrupt" fn syscall_handler(_stack_frame: &idt::ExceptionStackFrame) {
    println!("Syscall received.");    
}