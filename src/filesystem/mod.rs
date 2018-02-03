pub mod disk;
pub mod fat32;

use drivers::ata::Ata;
use self::disk::Disk;
use alloc::vec::Vec;
use alloc::string::String;

trait Filesystem {
    type FileType: File;
    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Self::FileType>>;
    fn read_file(&self, drive: &Disk, descriptor: FileDescriptor<Self::FileType>, buffer: &[u8], count: u64);
}

trait File {
    fn get_name(&self) -> String;
}

enum FileMode {
    Read,
    Write,
    ReadWrite,
}

struct FilePointer<T: File> {
    current: usize,
    file: T,
}

impl <T: File> FilePointer<T> {
    pub fn get_file(&self) -> &T {
        &self.file
    }
}

struct FileDescriptor<T: File> {
    id: u16,
    mode: FileMode,
    pointer: FilePointer<T>
}

struct ManagedFilesystem<'a, T: 'a + Filesystem> {
    filesystem: &'a T,
    drive: &'a Disk,
    descriptors: Vec<FileDescriptor<T::FileType>>,
}

impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    fn open_file(&mut self, file_name: &str) {
        let file_pointer = self.filesystem.open_file(self.drive, file_name).expect("Unable to get file pointer.");
        println!("Got File Pointer.");
        let descriptor = FileDescriptor {
            id: 1,
            mode: FileMode::ReadWrite,
            pointer: file_pointer,
        };
        self.descriptors.push(descriptor);
    }

    fn read_file(&self, descriptor: u16, buffer: &[u8], count: u64) {
        if let Some(descriptor) = self.descriptors.iter().find(|x| x.id == descriptor) {
            println!("File name is: {}", descriptor.pointer.get_file().get_name());
        } else {
            println!("Descriptor {} is not open.", descriptor);
        }
    }
}

pub fn detect() {
    let fat32 = unsafe { fat32::Fat32::new(&Ata::PRIMARY) };

    let mut fat = ManagedFilesystem {
        filesystem: &fat32,
        drive: &Ata::PRIMARY,
        descriptors: Vec::new(),
    };

    println!("\nOpening file \"{}\"", "testdir/testfile.txt");
    fat.open_file("testdir/testfile.txt");
    fat.read_file(1, &[0u8;1], 1);
    loop {};
}