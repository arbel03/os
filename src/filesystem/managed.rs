use filesystem::Filesystem;
use filesystem::descriptor::{ FileDescriptor, FilePointer };
use filesystem::disk::Disk;
use filesystem::File;
use filesystem::OpenError;
use alloc::Vec;

pub struct ManagedFilesystem<'a, T: Filesystem> {
    filesystem: T,
    drive: &'a Disk,
    descriptors: Vec<FileDescriptor<T::EntryType>>,
}

#[allow(dead_code)]
impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    pub fn new(filesystem: T, drive: &'a Disk) -> Self {
        ManagedFilesystem {
            filesystem: filesystem,
            drive: drive, 
            descriptors: Vec::new(),
        }
    }

    pub fn get_directory(&self, directory_path: &str) -> Option<T::EntryType> {
        self.filesystem.get_directory(self.drive, directory_path)
    }

    pub fn get_root_directory(&self) -> T::EntryType {
        self.filesystem.get_root_directory()
    }

    pub fn get_child_directories(&self, directory: &T::EntryType) -> Vec<T::EntryType> {
        self.filesystem.get_child_directories(self.drive, directory)
    }

    pub fn open_file(&mut self, file_name: &str) -> Result<usize, OpenError> {
        match self.filesystem.get_file(self.drive, file_name) {
            Ok(file) => {
                let mut lowest_index = self.descriptors.len();
                for (index, descriptor) in self.descriptors.iter().enumerate() {
                    if descriptor.get_id() as usize > index {
                        lowest_index = index;
                        break;
                    }
                }
                let descriptor = FileDescriptor::new(lowest_index, FilePointer::new(0, file));
                self.descriptors.insert(lowest_index, descriptor);
                Ok(lowest_index)
            },
            Err(open_error) => Err(open_error),
        }
    }

    pub fn seek(&mut self, descriptor: usize, new_current: usize) {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            let file_size = file_pointer.get_file().get_size();
            if new_current < file_size {
                file_pointer.set_current(new_current);
            } else {
                file_pointer.set_current(file_size - 1);
            }
        } else {
            println!("[KERNEL] Descriptor {} is not open.", descriptor);
        }
    }

    pub fn close_descriptor(&mut self, descriptor: usize) {
        let result = self.descriptors.iter().position(|x| x.get_id() == descriptor);
        if let Some(index) = result {
            self.descriptors.remove(index);
        } else {
            println!("[KERNEL] Descriptor {} is not open.", descriptor);
        }
    }

    pub fn read_file(&mut self, descriptor: usize, buffer: &mut [u8]) -> usize {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            if let Some(result) = self.filesystem.read_file(self.drive, file_pointer, buffer) {
                file_pointer.advance_current(result);
            } else {
                println!("[KERNEL] Unable to read file.");
            }
        } else {
            println!("[KERNEL] Descriptor {} is not open.", descriptor);
        }
        0
    }
}
