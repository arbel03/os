use filesystem::FILESYSTEM;

#[allow(dead_code)]
#[repr(usize)]
pub enum FilesystemErr {
    NoFile = 0x1,
}

// TODO: Add flags, `O_RDONLY` for example
pub unsafe fn open(file_name: &str) -> usize {
    if let Some(opened_descriptor) = FILESYSTEM.as_mut().unwrap().open_file(file_name) {
        return opened_descriptor;
    }
    1
}

pub unsafe fn seek(fd: usize, new_current: usize) -> usize {
    FILESYSTEM.as_mut().unwrap().seek(fd, new_current);
    1
}

// Reading contents of file to buffer
pub unsafe fn read(fd: usize, read_buffer: &mut [u8]) {
    FILESYSTEM.as_mut().unwrap().read_file(fd, read_buffer);
}