pub mod disk;
mod fat32;
mod file;
mod descriptor;

use self::file::*;
use self::descriptor::*;
use self::disk::Disk;
use drivers::ata::Ata;
use alloc::vec::Vec;

pub trait Filesystem {
    type FileType: File;
    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Self::FileType>>;
    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::FileType>, buffer: &mut [u8]) -> Option<usize>;
} 

pub struct ManagedFilesystem<'a, T: Filesystem> {
    filesystem: T,
    drive: &'a Disk,
    descriptors: Vec<FileDescriptor<T::FileType>>,
}

#[allow(dead_code)]
impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    pub fn open_file(&mut self, file_name: &str) -> Option<usize> {
        println!("Opening file: \"{}\"", file_name);
        if let Some(file_pointer) = self.filesystem.open_file(self.drive, file_name) {
            // println!("Got file pointer.");
            let mut lowest_index = self.descriptors.len();
            for (index, descriptor) in self.descriptors.iter().enumerate() {
                if descriptor.get_id() as usize > index {
                    lowest_index = index;
                    break;
                }
            }
            // println!("Creating new descriptor-{}.", lowest_index);
            let descriptor = FileDescriptor::new(lowest_index, FileMode::ReadWrite, file_pointer);
            self.descriptors.insert(lowest_index, descriptor);
            return Some(lowest_index);
        }
        println!("Unable to open file: \"{}\"", file_name);
        return None;
    }

    // TODO: Add error handling
    pub fn seek(&mut self, descriptor: usize, new_current: usize) {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            let file_size = file_pointer.get_file().get_size();
            if new_current < file_size {
                file_pointer.set_current(new_current);
            } else {
                println!("Trying to seek to outside of file bounds named {}", file_pointer.get_file().get_name());
            }
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
    }

    // TODO: Add error handling
    pub fn close_descriptor(&mut self, descriptor: usize) {
        let result = self.descriptors.iter().position(|x| x.get_id() == descriptor);
        if let Some(index) = result {
            self.descriptors.remove(index);
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
    }

    // TODO: Add error handling
    pub fn read_file(&mut self, descriptor: usize, buffer: &mut [u8]) -> usize {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            if let Some(result) = self.filesystem.read_file(self.drive, file_pointer, buffer) {
                file_pointer.advance_current(result);
            } else {
                println!("Unable to read file.");
            }
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
        0
    }

    // This is not a syscall
    pub fn get_current_offset(&self, descriptor: usize) -> usize {
        if let Some(descriptor) = self.descriptors.iter().find(|x| x.get_id() == descriptor) {
            let file_pointer = descriptor.get_pointer();
            return file_pointer.get_current();
        } else {
            return 0xFFFFFFFF;
        }
    }
}

// Fat Filesystem of the main disk.
pub static mut FILESYSTEM: Option<ManagedFilesystem<fat32::Fat32>> = None;

pub fn init() {
    unsafe {
        FILESYSTEM = Some(ManagedFilesystem {
            filesystem: fat32::Fat32::new(&Ata::PRIMARY),
            drive: &Ata::PRIMARY,
            descriptors: Vec::new(),
        });
    };
}