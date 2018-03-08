use super::file::*;

#[allow(dead_code)]
pub(in super) enum FileMode {
    Read,
    Write,
    ReadWrite,
}

#[allow(dead_code)]
pub(in super) struct FileDescriptor<T: File> {
    id: usize,
    mode: FileMode,
    pointer: FilePointer<T>
}

impl <T: File> FileDescriptor<T> {
    pub fn new(id: usize, mode: FileMode, pointer: FilePointer<T>) -> Self {
        FileDescriptor {
            id: id, 
            mode: mode, 
            pointer: pointer
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_pointer(&self) -> &FilePointer<T> {
        &self.pointer
    }

    pub fn get_pointer_mut(&mut self) -> &mut FilePointer<T> {
        &mut self.pointer
    }
}
