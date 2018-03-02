#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub(in super) struct Registers {
    pub edi: usize,
    pub esi: usize,
    pub edx: usize,
    pub ecx: usize,
    pub ebx: usize,
    pub eax: usize,
}