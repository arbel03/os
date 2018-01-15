mod idt;
use dtables::{TableDescriptor, lidt};
use drivers;

static mut IDT: [idt::IdtEntry; 256] = [idt::IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        IDT[8] = idt::IdtEntry::new(double_fault as u32);
        IDT[33] = idt::IdtEntry::new(keyboard_irq as u32); 
        IDT[5] = idt::IdtEntry::new(bound_range_exceeded as u32);             
        IDT[13] = idt::IdtEntry::new(general_protection_fault as u32);       
        IDT[46] = idt::IdtEntry::new(primary_ata_controller as u32);       
        
        let idtr = TableDescriptor::new(&IDT);
        lidt(&idtr);

        // Enable hardware interrupts
        asm!("sti");
    }
}

extern "x86-interrupt" fn primary_ata_controller(_stack_frame: &idt::ExceptionStackFrame) {
    println!("Primary ATA controller interrupted.");
    drivers::send_eoi(true);
}

extern "x86-interrupt" fn keyboard_irq(_stack_frame: &idt::ExceptionStackFrame) {
    if let Some(c) = drivers::keyboard::getc() {
        print!("{}", c);
    }
    drivers::send_eoi(false);
}

extern "x86-interrupt" fn general_protection_fault(stack_frame: &idt::ExceptionStackFrame, error_code: u32) {
    println!("Exception! General Protection Fault.");
    println!("Error code: {:b}", error_code);
    if error_code != 0 {
        let tbl_num = (error_code & 6) >> 1;
        println!("Error in {}", if tbl_num == 1 { "IDT" } else { "GDT" });
        println!("Error in index: {}", error_code >> 3);
    }
    println!("{}", stack_frame);
    loop {};
}

extern "x86-interrupt" fn bound_range_exceeded(stack_frame: &mut idt::ExceptionStackFrame) {
    println!("Exception! Bound Range Exceeded.");
    println!("{}", stack_frame);
    loop {};
}

extern "x86-interrupt" fn double_fault(stack_frame: &idt::ExceptionStackFrame, error_code: u32) {
    println!("Exception! Double Fault.");
    println!("Error Code: {:b}", error_code);
    println!("{}", stack_frame);
    loop {};
}
