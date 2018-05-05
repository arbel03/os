pub mod disk;
pub mod fat32;
mod descriptor;
mod managed;

pub use self::descriptor::{ FilePointer, File };
use self::managed::ManagedFilesystem;
use self::disk::Disk;
use alloc::Vec;
use drivers::ata::Ata;

#[repr(usize)]
#[derive(Debug)]
pub enum OpenError {
    FileNotFound = 0xffffffff,
    NotAFile = 0xfffffffe,
    DescriptorAlreadyOpen = 0xfffffffd,
}

pub trait Filesystem {
    type EntryType: File;

    fn get_root_directory(&self) -> Self::EntryType;
    fn get_child_directories(&self, drive: &Disk, directory: &Self::EntryType) -> Vec<Self::EntryType>;
    fn get_directory(&self, drive: &Disk, directory_path: &str) -> Option<Self::EntryType>;
    fn get_file(&self, drive: &Disk, file_path: &str) -> Result<Self::EntryType, OpenError>;
    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::EntryType>, buffer: &mut [u8]) -> Option<usize>;
} 

// Fat Filesystem of the main disk.
pub static mut FILESYSTEM: Option<ManagedFilesystem<fat32::Fat32>> = None;

pub fn init() {
    unsafe {
        FILESYSTEM = Some(ManagedFilesystem::new(fat32::Fat32::new(&Ata::PRIMARY),  &Ata::PRIMARY));
    };
}