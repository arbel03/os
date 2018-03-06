use syscall;

pub unsafe fn init() {
    use interrupts::{ idt, IDT };
    // Set handler for interrupt 64,
    // Syscall Interrupt[0x80] - (Exceptions[32d] + Hardware Interrupts[32d]) = 0x40
    IDT.interrupts[0x40] = idt::IdtEntry::new(syscall_handler as u32);
}

#[derive(Debug, Copy, Clone)]
#[repr(packed, C)]
struct Registers {
    pub edi: usize,
    pub esi: usize,
    pub edx: usize,
    pub ecx: usize,
    pub ebx: usize,
    pub eax: usize,
}

#[naked]
#[inline(never)]
pub unsafe extern fn syscall_handler() {
    #[inline(never)]
    unsafe extern fn syscall(regs: &Registers) {
        let result = syscall::syscall(regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi);
        asm!("mov eax, $0":: "m"(result) :: "intel");
    }
    
    // Get reference to stack variables
    let esp: usize;

    asm!("
        push eax
        push ebx
        push ecx
        push edx
        push esi
        push edi"
        : "={esp}"(esp) ::: "intel");
    
    syscall(&mut *(esp as *mut Registers));

    asm!("
        pop edi
        pop esi
        pop edx
        pop ecx
        pop ebx
        add esp, 4
        iretd" 
        :::: "intel");
}
