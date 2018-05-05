use alloc::string::String;

pub trait File {
    fn get_name(&self) -> String;
    fn get_size(&self) -> usize; 
}

pub struct FilePointer<T: File> {
    current: usize,
    file: T,
}

#[allow(dead_code)]
impl <T: File> FilePointer<T> {
    pub fn new(current: usize, file: T) -> Self {
        FilePointer {
            current: current,
            file: file,
        }
    }

    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn advance_current(&mut self, amount: usize) {
        self.current += amount
    }

    pub fn set_current(&mut self, new_current: usize) {
        self.current = new_current
    }

    pub fn get_file(&self) -> &T {
        &self.file
    }
}

#[allow(dead_code)]
pub(in super) struct FileDescriptor<T: File> {
    id: usize,
    pointer: FilePointer<T>
}

impl <T: File> FileDescriptor<T> {
    pub fn new(id: usize, pointer: FilePointer<T>) -> Self {
        FileDescriptor {
            id: id, 
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