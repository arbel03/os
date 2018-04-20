pub mod syscall;
mod idt;
mod exceptions;

use self::exceptions::*;
use drivers;

static mut IDT: idt::Idt = idt::Idt::new();

macro_rules! save_registers {
    () => {{
        asm!("
        push eax
        push ebx
        push ecx
        push edx
        push esi
        push edi
        push ebp

        push ds
        push gs
        push fs
        push es
        
        mov bx, 0x10
        mov ds, bx
        mov fs, bx
        mov es, bx
        mov gs, bx
        " :::: "intel", "volatile");
    }};
}

macro_rules! restore_registers {
    () => {{
        asm!("
        pop es
        pop fs
        pop gs
        pop ds

        pop ebp
        pop edi
        pop esi
        pop edx
        pop ecx
        pop ebx
        pop eax
        " :::: "intel", "volatile");
    }};
}

macro_rules! define_interrupt {
    ($b:ident, $pl:expr) => {{
        #[naked]
        fn wrapper() {
            unsafe {
                save_registers!();
                asm!("
                mov ebx, $0

                mov eax, esp
                add eax, 10*4
                push eax

                call ebx
                add esp, 4
                " :: "r"($b as extern "C" fn(&idt::ExceptionStackFrame) as usize) :: "intel", "volatile");
                restore_registers!();
                asm!("iretd" :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        };
        idt::IdtEntry::new(wrapper as u32, $pl)
    }};
}

macro_rules! define_interrupt_with_error_code {
    ($b:ident, $pl:expr) => {{
        #[naked]
        fn wrapper() {
            unsafe {
                save_registers!();
                asm!("
                mov ebx, $0

                mov eax, esp
                add eax, 10*4

                mov ecx, [eax]
                add eax, 4

                push ecx
                push eax                
                call ebx
                add esp, 8
                " :: "r"($b as extern "C" fn(&idt::ExceptionStackFrame, u32) as usize) :: "intel", "volatile");
                restore_registers!();
                asm!("add esp, 4
                iretd" :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        };
        idt::IdtEntry::new(wrapper as u32, $pl)
    }};
}

pub fn init() {
    unsafe {
        // Exceptions
        IDT.exceptions.double_fault = define_interrupt_with_error_code!(double_fault, 0);
        IDT.exceptions.general_protection_fault = define_interrupt_with_error_code!(general_protection_fault, 0);
        IDT.exceptions.invalid_opcode = define_interrupt!(invalid_opcode, 0);
        IDT.exceptions.invalid_tss = define_interrupt_with_error_code!(invalid_tss, 0);

        // Hardware interrupts       
        IDT.set_hardware_interrupt(1, define_interrupt!(keyboard_irq, 0));              
        IDT.set_hardware_interrupt(14, define_interrupt!(primary_ata_controller, 0));

        // Setup syscalls
        syscall::init();

        IDT.load();
    }
}

pub fn enable() {
    unsafe {
        asm!("sti");
    }
}

pub fn disable() {
    unsafe {
        asm!("cli");
    }
}

extern "C" fn invalid_opcode(_stack_frame: &idt::ExceptionStackFrame) {
    println!("Invalid opcode.");
    loop {};
}

extern "C" fn primary_ata_controller(_stack_frame: &idt::ExceptionStackFrame) {
    drivers::send_eoi(true);
}

extern "C" fn keyboard_irq(_stack_frame: &idt::ExceptionStackFrame) {
    // loop {};
    if let Some(c) = drivers::keyboard::getc() {
        print!("{}", c);
    }
    drivers::send_eoi(false);
}