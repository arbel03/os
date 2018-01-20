pub mod disk;
pub mod fat;

use self::fat::Fat32;
use drivers::ata::Ata;
use self::disk::Disk;

struct ManagedFilesystem<'a, T: 'a +  Filesystem> {
    filesystem: &'a T,
    drive: &'a Disk,
}

impl <'a, T: Filesystem>  ManagedFilesystem<'a, T> {
    fn open_file(&mut self, file_name: &str) {
        let file_pointer = self.filesystem.open_file(self.drive, file_name);
    }
}

trait Filesystem {
    type FileType;
    fn open_file(&self, drive: &Disk, file_name: &str) -> FilePointer<Self::FileType>;
    // fn close_descriptor(&mut self, descriptor: FileDescriptor<Self::FileType>);
    fn read_file(&self, drive: &Disk, descriptor: FilePointer<Self::FileType>, buffer: &[u8], count: u64);
}

enum FileMode {
    Read,
    Write,
}

struct FilePointer<T> {
    current_position: u64,
    file: T,
}

struct FileDescriptor<T> {
    id: u16,
    mode: FileMode,
    pointer: FilePointer<T>
}

pub fn detect() {
    unsafe {
        use self::fat::Ebpb;

        let mut x: [u8;512] = [0u8;512];
        Ata::PRIMARY.read(0, &mut x).unwrap();
        let bpb = &*(x.as_ptr() as *const Ebpb);
        let fat32: Fat32 = Fat32::new(&bpb);

        let mut fat = ManagedFilesystem {
            filesystem: &fat32,
            drive: &Ata::PRIMARY,
        };

        fat.open_file("/");

        // let mut x = [0u8;512];
        // Ata::PRIMARY.read(ebpb.bpb.reserved_sectors as u64 - 1, &mut x).unwrap();
        // println!("Printing FAT:");

        // for (i, n) in x[..256].iter().enumerate() {
        //     print!("{:02x} ", n);
        //     if (i+1) % 16 == 0 {
        //         print!("\n");
        //     } else if (i+1) % 8 == 0 {
        //         print!("  ");
        //     }
        // }
        // print!("\n");
    }
    loop {};
}