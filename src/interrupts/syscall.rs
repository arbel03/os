use syscall;

extern {
    fn syscall_handler();
}

pub unsafe fn init() {
    use interrupts::{ idt, IDT };
    // Set handler for interrupt 64,
    // Syscall Interrupt[0x80] - (Exceptions[32d] + Hardware Interrupts[32d]) = 0x40
    IDT.interrupts[0x40] = idt::IdtEntry::new(syscall_handler as u32, 3);

    // Set task gate
    use memory::gdt::{Gdt, DescriptorType};
    use memory::GDT;
    IDT.interrupts[0x42] = idt::IdtEntry::new_task_gate(GDT.get_selector(DescriptorType::TssDescriptor, 0));
}

#[derive(Debug)]
#[repr(packed, C)]
pub struct SyscallStack {
    pub edi: usize,
    pub esi: usize,
    pub edx: usize,
    pub ecx: usize,
    pub ebx: usize,
    pub eax: usize,
}

#[no_mangle]
pub unsafe extern fn syscall_handler_inner(regs: &SyscallStack) {
    use memory::utils::*;
    use memory::GDT;
    use memory::gdt::{ Gdt, DescriptorType };
    load_ds(GDT.get_selector(DescriptorType::KernelData, 0));
    load_es(GDT.get_selector(DescriptorType::KernelData, 0));
    load_fs(GDT.get_selector(DescriptorType::KernelData, 0));
    load_gs(GDT.get_selector(DescriptorType::KernelData, 0));

    println!("Received syscall, registers: {:?}", regs);
    let result = syscall::syscall(regs.eax, regs.ebx, regs.ecx, regs.edx, regs.esi, regs.edi);
    asm!("mov eax, $0" :: "m"(result) :: "intel");
}
