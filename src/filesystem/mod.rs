pub mod disk;
pub mod fat32;

use drivers::ata::Ata;
use self::disk::Disk;
use alloc::string::String;

struct ManagedFilesystem<'a, T: 'a + Filesystem> {
    filesystem: &'a T,
    drive: &'a Disk,
}

impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    fn open_file(&mut self, file_name: &str) {
        let file_pointer = self.filesystem.open_file(self.drive, file_name);
    }
}

trait Filesystem {
    fn open_file(&self, drive: &Disk, file_name: &str) -> FilePointer;
    fn read_file(&self, drive: &Disk, descriptor: FileDescriptor, buffer: &[u8], count: u64);
}

enum FileMode {
    Read,
    Write,
}

struct FilePointer {
    current_position: u64,
    file_name: String,
}

struct FileDescriptor {
    id: u16,
    mode: FileMode,
    pointer: FilePointer
}

pub fn detect() {
    let fat32 = unsafe { fat32::Fat32::new(&Ata::PRIMARY) };

    let mut fat = ManagedFilesystem {
        filesystem: &fat32,
        drive: &Ata::PRIMARY,
    };

    fat.open_file("/");
    loop {};
}