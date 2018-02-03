mod directory;
mod table;

use self::directory::*;
use self::table::*;
use super::{ Filesystem, FilePointer, FileDescriptor, File };
use super::disk::Disk;

use alloc::Vec;
use alloc::string::String;
use core::str::Split;
use core::slice;


// Fat32 implementaion
pub struct Fat32 {
    ebpb: Ebpb,
}

impl Fat32 {
    pub unsafe fn new(disk: &Disk) -> Self {
        let mut x: [u8;512] = [0u8;512];
        disk.read(0, &mut x).expect("Error reading EBPB from disk."); // Read the first sector into x
        let ebpb = (*(x.as_ptr() as *const Ebpb)).clone();
        Fat32 {
            ebpb: ebpb,
        }
    }

    fn get_first_data_sector(&self) -> u64 {
        self.ebpb.bpb.reserved_sectors_count as u64 + (self.ebpb.bpb.table_count as u32 * self.ebpb.sectors_per_fat) as u64
    }

    fn get_bytes_in_cluster(&self) -> u32 {
        //self.ebpb.bpb.sectors_per_cluster as u32 * self.ebpb.bpb.bytes_per_sector as u32
        self.ebpb.bpb.sectors_per_cluster as u32 * self.ebpb.bpb.bytes_per_sector as u32
    }

    fn first_sector_of_cluster(&self, cluster: u32) -> u64 {
        ((cluster-2) * (self.ebpb.bpb.sectors_per_cluster as u32)) as u64 + self.get_first_data_sector()
    }

    fn read_directories_from_cluster(&self, drive: &Disk, cluster: Cluster, directories: &mut Vec<Directory>) {
        let mut temp_name: Option<String> = None;
        let mut buffer = vec![0u8; self.get_bytes_in_cluster() as usize];

        let sectors_read = unsafe { drive.read(self.first_sector_of_cluster(cluster.0), &mut buffer) }.expect("Error reading from disk.") as usize;
        let directories_slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const FatDirectory, (sectors_read * self.ebpb.bpb.bytes_per_sector as usize / 32) as usize) };

        for directory in directories_slice {
            // If the first byte of the directory entry is 0, there are no more directories.
            // If the first byte of the directory entry is 0xE5, the directory is not used.
            if directory.name[0] == 0 {
                break;
            } else if directory.name[0] == 0xE5 {
                continue;
            }

            if directory.is_lfn() {
                let lfn_directory = unsafe { *(directory as *const _ as *const LongFileName) };
                let long_file_name = lfn_directory.get_name();
                temp_name = Some(long_file_name);
            } else {
                if let Some(stored_name) = temp_name {
                    directories.push(Directory::new(stored_name, *directory));
                    temp_name = None;
                } else {
                    directories.push(Directory::new(directory.get_short_name(), *directory));
                }
            }
        }
    }

    fn read_cluster_chain(&self, drive: &Disk, first_cluster: u32, directories: &mut Vec<Directory>) {
        let cluster_chain = ClusterChain::new(Cluster(first_cluster), self, drive);
        for cluster in cluster_chain {
            self.read_directories_from_cluster(drive, cluster, directories);
        }
    }

    fn read_folder(&self, drive: &Disk, cluster: u32) -> Vec<Directory> {
        let mut directories: Vec<Directory> = Vec::new();
        self.read_cluster_chain(drive, cluster, &mut directories);
        return directories;
    }

    fn find_file(&self, drive: &Disk, cluster: u32, path: &mut Split<&str>) -> Option<Directory> {
        if let Some(component) = path.next() {
            let current_dirs = self.read_folder(drive, cluster);
            let dir = current_dirs
                    .iter()
                    .find(|dir| dir.get_name() == component)
                    .expect(&format!("Folder {} not found.", component))
                    .clone();
            if dir.is_folder() {
                return self.find_file(drive, dir.get_cluster(), path);
            } else {
                return Some(dir);
            }
        } else {
            // Reached the end of path iterator
            return None;
        }
    }
}

impl Filesystem for Fat32 {
    type FileType = Directory;

    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Directory>> {
        let mut path_components = file_name.split("/");
        // let directories = self.read_folder(drive, self.ebpb.root_dir_cluster);
        // println!("{:?}", directories);
        if let Some(file) = self.find_file(drive, self.ebpb.root_dir_cluster, &mut path_components) {
           // If the file is really a file
            return Some(FilePointer {
                current: 0,
                file: file,
            });
        }
        None
    }

    fn read_file(&self, drive: &Disk, descriptor: FileDescriptor<Directory>, buffer: &[u8], count: u64) {

    }
}