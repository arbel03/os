use filesystem::FILESYSTEM;

#[repr(usize)]
pub enum FilesystemErr {
    NoFile = 0x1,
}

// TODO: Add flags, `O_RDONLY` for example
pub fn open(file_name: &str) -> Result<usize, usize> {
    if let Some(opened_descriptor) = unsafe { FILESYSTEM.as_mut().unwrap().open_file(file_name) } {
        return Ok(opened_descriptor as usize);
    }
    Err(FilesystemErr::NoFile as usize)
}