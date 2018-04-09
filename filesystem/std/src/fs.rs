pub fn open(file_path: &str) -> usize {
    use syscall::syscall2;
    // SYSCALL(SYS_FOPEN, ptr, size)
    unsafe {
        syscall2(0x1, file_path.as_ptr() as usize, file_path.len())
    }
}