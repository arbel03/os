#[repr(packed)]
#[derive(Debug)]
pub struct Stat {
    pub directory_name_length: usize,
    pub directory_size: usize,
    pub is_folder: bool,
    pub child_nodes_count: usize,
}

pub fn open(file_path: &str) -> usize {
    use syscalls::syscall::syscall2;
    // SYSCALL(SYS_FOPEN, ptr, size)
    unsafe {
        syscall2(0x1, file_path.as_ptr() as usize, file_path.len())
    }
}

pub fn read(fd: usize, buffer: &mut [u8]) -> usize {
    use syscalls::syscall::syscall3;
    // SYSCALL(SYS_READ, fd, ptr, size)
    unsafe {
        syscall3(0x3, fd, buffer.as_ptr() as usize, buffer.len())
    }
}

pub fn stat(parent_directory: &str, child_node: usize) -> Stat {
    // SYSCALL(SYS_STAT, ptr, size, stat_structure_ptr, child_node)
    use syscalls::syscall::syscall4;

    let mut stat = Stat {
        directory_name_length: 0,
        directory_size: 0,
        is_folder: false,
        child_nodes_count: 0,
    };

    unsafe {
        syscall4(0x04, parent_directory.as_ptr() as usize, parent_directory.len(), &stat as *const Stat as usize, child_node);
    }
    stat
}

pub fn read_name(parent_directory: &str, read_buffer: &mut [u8], child_node: usize) {
    // SYSCALL(SYS_STAT, ptr, size, read_buffer_ptr, read_buffer_size, child_node)
    use syscalls::syscall::syscall5;

    unsafe {
        syscall5(0x08, parent_directory.as_ptr() as usize, parent_directory.len(), read_buffer.as_ptr() as usize, read_buffer.len(), child_node);
    }
}