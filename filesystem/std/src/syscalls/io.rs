pub fn printf(string: &str) -> usize {
    use syscalls::syscall::syscall2;
    // SYSCALL(SYS_FOPEN, ptr, size)
    unsafe {
        syscall2(0x2, string.as_ptr() as usize, string.len())
    }
}