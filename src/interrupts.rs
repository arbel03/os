use dtables::Interrupts::*;
use dtables::TableDescriptor;
use dtables::lidt;

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

pub fn init() {
    unsafe {
        let IDTR = TableDescriptor::new(&IDT);
        lidt(&IDTR);

        IDT[3] = IdtEntry::new(breakpoint_handler as u32);
        
        // TODO: Fix triple fault
        // http://wiki.osdev.org/I_Can't_Get_Interrupts_Working
        //asm!("int 3" :::: "intel");
    }
}

extern "C" fn breakpoint_handler() {
    println!("Exception!: Breakpoint");
    loop {}
}

