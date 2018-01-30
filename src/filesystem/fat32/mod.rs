mod directory;
mod table;

use self::directory::*;
use self::table::*;
use super::{ Filesystem, FilePointer, FileDescriptor};
use super::disk::Disk;

use alloc::Vec;
use alloc::string::String;
use core::slice;


// Fat32 implementaion
pub struct Fat32<'a> {
    ebpb: &'a Ebpb,
}

impl <'a> Fat32<'a> {
    pub fn new(disk: &Disk) -> Self {
        let mut x: [u8;512] = [0u8;512];
        unsafe {
            disk.read(0, &mut x).unwrap();
            let ebpb = &*(x.as_ptr() as *const Ebpb);
            Fat32 {
                ebpb: ebpb,
            }
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
        self.get_first_data_sector() + ((cluster-2) * (self.ebpb.bpb.sectors_per_cluster as u32)) as u64
    }

    fn get_total_clusters(&self) -> u32 {
        let data_sectors = self.ebpb.bpb.total_sectors_large as usize - (self.ebpb.bpb.reserved_sectors_count as usize + self.ebpb.bpb.table_count as usize *32);
        return data_sectors as u32 / self.ebpb.bpb.sectors_per_cluster as u32;
    }

    fn read_directories_from_cluster(&self, drive: &Disk, cluster: Cluster, directories: &mut Vec<Directory>) {
        let mut temp_name: Option<String> = None;
        let mut buffer = vec![0u8; self.get_bytes_in_cluster() as usize];
        //println!("Vec at {}, size: {}.", buffer.as_ptr() as u32, buffer.len());
        let read_dirs_count = unsafe {
            let result = drive.read(self.first_sector_of_cluster(cluster.0), &mut buffer);
            match result {
                Ok(amount_read) => amount_read as usize,
                Err(err_msg) => panic!("{}", err_msg),
            }
        };  
        let directories_slice = unsafe { slice::from_raw_parts(buffer.as_ptr() as *const FatDirectory, (read_dirs_count * 512 / 32) as usize) };

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
}

impl <'a> Filesystem for Fat32<'a> {
    fn open_file(&self, drive: &Disk, file_name: &str) -> FilePointer {
        let root_dirs = self.read_folder(drive, self.ebpb.root_dir_cluster);
        println!("{}", root_dirs[0].get_name());
        if root_dirs[0].is_folder() {
            let subdirs = self.read_folder(drive, root_dirs[0].get_cluster());
            for dir in subdirs {
                println!("    {}", dir.get_name());
            }
        }

        FilePointer {
            current_position: 0,
            file_name: String::from("Directory"),
        }
    }

    fn read_file(&self, drive: &Disk, descriptor: FileDescriptor, buffer: &[u8], count: u64) {

    }
}