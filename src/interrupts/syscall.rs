use syscall;

extern {
    fn syscall_handler();
}

pub unsafe fn init() {
    use interrupts::{ idt, IDT };
    // Set handler for interrupt 64,
    // Syscall Interrupt[0x80] - (Exceptions[32d] + Hardware Interrupts[32d]) = 0x40
    IDT.interrupts[0x40] = idt::IdtEntry::new(syscall_handler as u32);
}

#[derive(Debug, Copy, Clone)]
#[repr(packed, C)]
pub struct Registers {
    pub edi: usize,
    pub esi: usize,
    pub edx: usize,
    pub ecx: usize,
    pub ebx: usize,
    pub eax: usize,
}

#[no_mangle]
pub unsafe extern fn syscall_handler_inner(regs: &Registers) { 
    let result = syscall::syscall(regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi);
    asm!("mov eax, $0" :: "m"(result) :: "intel");
}
