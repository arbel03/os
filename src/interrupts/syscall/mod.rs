mod calls;
mod regs;

use self::calls::*;
use interrupts::{ idt, IDT };

pub unsafe fn init() {
    // Set handler for interrupt 64,
    // Syscall Interrupt[0x80] - (Exceptions[32d] + Hardware Interrupts[32d]) = 0x40
    IDT.interrupts[0x40] = idt::IdtEntry::new(syscall_handler as u32);
}

#[naked]
pub unsafe extern fn syscall_handler() {
    asm!("
        push eax
        push ebx
        push ecx
        push edx
        push esi
        push edi"
        :::: "intel", "volatile");
    
    // Get reference to stack variables
    let esp: usize;
    asm!("" : "={esp}"(esp) : : : "intel", "volatile");
    let regs = &mut *(esp as *mut regs::Registers);
    let result = syscall(regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi);

    asm!("pop edi
        pop esi
        pop edx
        pop ecx
        pop ebx
        pop eax
        mov eax, $0
        iretd" 
        :: "m"(result) :: "intel", "volatile");
}

fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    println!("Syscall received. ({}, {}, {}, {}, {}, {})", a, b, c, d, e, f);
    return a;
}
