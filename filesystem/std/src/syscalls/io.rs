pub fn printf(string: &str) -> usize {
    use syscalls::syscall::syscall2;
    // SYSCALL(SYS_FOPEN, ptr, size)
    unsafe {
        syscall2(0x2, string.as_ptr() as usize, string.len())
    }
}

pub fn getc() -> char {
    use syscalls::syscall::syscall0;
    // SYSCALL(IO_GETC)
    unsafe {
        syscall0(0x5) as u8 as char
    }
}