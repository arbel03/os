pub fn open(file_path: &str) -> usize {
    use syscall::syscall2;
    // SYSCALL(SYS_FOPEN, ptr, size)
    unsafe {
        syscall2(0x1, file_path.as_ptr() as usize, file_path.len())
    }
}

pub fn read(fd: usize, buffer: &mut [u8]) -> usize {
    use syscall::syscall3;
    // SYSCALL(SYS_READ, fd, ptr, size)
    unsafe {
        syscall3(0x3, fd, buffer.as_ptr() as usize, buffer.len())
    }
}

pub fn filesz(fd: usize) -> usize {
    use syscall::syscall1;
    // SYSCALL(SYS_FILESZ, fd)
    unsafe {
        syscall1(0x4, fd)
    }
}