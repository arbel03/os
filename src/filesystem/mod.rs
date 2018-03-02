pub mod disk;
pub mod fat32;

use drivers::ata::Ata;
use self::disk::Disk;
use alloc::vec::Vec;
use alloc::string::String;

trait Filesystem {
    type FileType: File;
    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Self::FileType>>;
    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::FileType>, buffer: &mut [u8]) -> Option<usize>;
} 

trait File {
    fn get_name(&self) -> String;
    fn get_size(&self) -> usize; 
}

#[allow(dead_code)]
enum FileMode {
    Read,
    Write,
    ReadWrite,
}

struct FilePointer<T: File> {
    current: usize,
    file: T,
}

#[allow(dead_code)]
impl <T: File> FilePointer<T> {
    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn advance_pointer(&mut self, amount: usize) {
        self.current += amount
    }

    pub fn get_file(&self) -> &T {
        &self.file
    }
}

#[allow(dead_code)]
struct FileDescriptor<T: File> {
    id: u16,
    mode: FileMode,
    pointer: FilePointer<T>
}

impl <T: File> FileDescriptor<T> {
    pub fn get_id(&self) -> u16 {
        self.id
    }

    #[allow(dead_code)]
    pub fn get_pointer(&self) -> &FilePointer<T> {
        &self.pointer
    }

    pub fn get_pointer_mut(&mut self) -> &mut FilePointer<T> {
        &mut self.pointer
    }
}

struct ManagedFilesystem<'a, T: Filesystem> {
    filesystem: T,
    drive: &'a Disk,
    descriptors: Vec<FileDescriptor<T::FileType>>,
}

impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    fn open_file(&mut self, file_name: &str) -> Option<u16> {
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
            let descriptor = FileDescriptor {
                id: lowest_index as u16,
                mode: FileMode::ReadWrite,
                pointer: file_pointer,
            };
            self.descriptors.insert(lowest_index, descriptor);
            return Some(lowest_index as u16);
        }
        println!("Unable to get file pointer.");
        return None;
    }

    #[allow(dead_code)]
    fn close_descriptor(&mut self, descriptor: u16) {
        let result = self.descriptors.iter().position(|x| x.get_id() == descriptor);
        if let Some(index) = result {
            self.descriptors.remove(index);
            // println!("Closed descriptor {}.", index);
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
    }

    fn read_file(&mut self, descriptor: u16, buffer: &mut [u8]) {
        if let Some(descriptor) = self.descriptors.iter_mut().find(|x| x.id == descriptor) {
            let file_pointer = descriptor.get_pointer_mut();
            if let Some(result) = self.filesystem.read_file(self.drive, file_pointer, buffer) {
                file_pointer.advance_pointer(result);
            } else {
                println!("Unable to read file.");
            }
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
    }
}

// Fat Filesystem of the main disk.
static mut FAT: Option<ManagedFilesystem<fat32::Fat32>> = None;

pub fn init() {
    unsafe {
        FAT = Some(ManagedFilesystem {
            filesystem: fat32::Fat32::new(&Ata::PRIMARY),
            drive: &Ata::PRIMARY,
            descriptors: Vec::new(),
        });
    };
    
    // let path = "BIN/PRINT   O";
    // println!("");
    // println!("Opening file \"{}\".", path);
    // if let Some(opened_descriptor) = fat.open_file(path) {
    //     println!("Printing contents of file:");

    //     let mut buffer = [0u8; 512];
    //     fat.read_file(opened_descriptor, &mut buffer); 
    //     use core::str;
    //     println!("{}", unsafe { str::from_utf8_unchecked(&buffer) });
    // }
}