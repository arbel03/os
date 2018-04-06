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

        push ds
        push gs
        push fs
        push es" :::: "intel", "volatile");
    }};
}

macro_rules! restore_registers {
    () => {{
        asm!("
        pop es
        pop fs
        pop gs
        pop ds

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
                mov bx, 0x10
                mov ds, bx
                mov fs, bx
                mov es, bx
                mov gs, bx

                mov ebx, $0

                mov eax, esp
                add eax, 6*4
                add eax, 3*4
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
                mov bx, 0x10
                mov ds, bx
                mov fs, bx
                mov es, bx
                mov gs, bx

                mov ebx, $0

                mov eax, esp
                add eax, 9*4

                mov ecx, [eax]
                inc eax

                push ecx
                push eax                
                call ebx
                add esp, 4
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
        IDT.exceptions.double_fault =  define_interrupt_with_error_code!(double_fault, 0);
        IDT.exceptions.general_protection_fault = define_interrupt_with_error_code!(general_protection_fault, 0);

        // Hardware interrupts       
        IDT.set_hardware_interrupt(1, define_interrupt!(keyboard_irq, 0));              
        IDT.set_hardware_interrupt(14, define_interrupt!(primary_ata_controller, 0));

        // Setup syscalls
        syscall::init();

        IDT.load();
        // Enable hardware interrupts
        asm!("sti");
    }
}

extern "C" fn primary_ata_controller(_stack_frame: &idt::ExceptionStackFrame) {
    drivers::send_eoi(true);
}

extern "C" fn keyboard_irq(_stack_frame: &idt::ExceptionStackFrame) {
    // println!("{}", _stack_frame);
    if let Some(c) = drivers::keyboard::getc() {
        print!("{}", c);
    }
    drivers::send_eoi(false);
}