mod directory;
mod table;

use self::directory::*;
use self::table::*;
use super::{ Filesystem, FilePointer, File };
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

    fn get_bytes_in_cluster(&self) -> usize {
        //self.ebpb.bpb.sectors_per_cluster as u32 * self.ebpb.bpb.bytes_per_sector as u32
        self.ebpb.bpb.sectors_per_cluster as usize * self.ebpb.bpb.bytes_per_sector as usize
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
                let long_file_name = unsafe { directory.get_long_name() };
                if temp_name != None {
                    // If a long file name is in the buffer and the current directory is another long file name, 
                    // apply it to the previously stored file name.
                    temp_name = Some(format!("{}{}", long_file_name, temp_name.unwrap()));
                } else {
                    temp_name = Some(long_file_name);
                }
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
            let mut dir: &Directory;
            // let names = current_dirs.iter().map(|x| x.get_name()).collect::<Vec<String>>();
            // println!("Searching {} in {:?}", component, names);
            if let Some(found_dir) = current_dirs.iter().find(|dir| dir.get_name() == component) {
                dir = found_dir;
            } else {
                dir = current_dirs.iter().find(|dir| { 
                    use alloc::string::ToString;
                    return dir.get_name() == component.to_string().to_uppercase();
                }).unwrap();
            }
            if dir.get_fat_dir().is_folder() {
                return self.find_file(drive, dir.get_fat_dir().get_cluster(), path);
            } else {
                return Some(dir.clone());
            }
        } else {
            // Reached the end of path iterator
            return None;
        }
    }

    fn read_clusters(&self, drive: &Disk, fat_file: &FatDirectory, start_cluster: usize, buffer: &mut [u8]) -> usize {
        use core::cmp::min;
        let cluster_size = self.get_bytes_in_cluster() as usize;
        // Getting the file size in clusters
        let file_size = fat_file.get_size() / cluster_size + 1;
        let read_size = buffer.len() / cluster_size;

        // If we don't try to read more than the file size
        // Get a cluster chain for the file
        let first_cluster = fat_file.get_cluster();
        let mut cluster_chain = ClusterChain::new(Cluster(first_cluster), self, drive);
        if start_cluster != 0 {
            // Consuming `starting_cluster` elements from the cluster chain
            cluster_chain.nth(start_cluster-1);
        }

        let mut part = 0;
        // Getting the cluster we should read from, if its out of the borders of the chain, return None
        for cluster in cluster_chain {
            if part >= min(file_size, read_size) {
                break;
            }
            let mut temp_buffer = vec![0u8;cluster_size];
            unsafe { drive.read(self.first_sector_of_cluster(cluster.0), &mut temp_buffer).unwrap(); }

            buffer[part*cluster_size..(part+1)*cluster_size].clone_from_slice(&temp_buffer);
            part += 1;
        }

        part*cluster_size
    }
}

impl Filesystem for Fat32 {
    type FileType = Directory;

    fn open_file(&self, drive: &Disk, file_name: &str) -> Option<FilePointer<Directory>> {
        let mut path_components = file_name.split("/");
        if let Some(file) = self.find_file(drive, self.ebpb.root_dir_cluster, &mut path_components) {
           // If the file is really a file
            return Some(FilePointer::new(0, file));
        }
        None
    }

    fn read_file(&self, drive: &Disk, file_pointer: &FilePointer<Self::FileType>, buffer: &mut [u8]) -> Option<usize> {
        let first_cluster = file_pointer.get_current() / self.get_bytes_in_cluster() as usize;
        let end_cluster = (file_pointer.get_current() + buffer.len()) / self.get_bytes_in_cluster();

        // println!("Reading from {} to {}", first_cluster, end_cluster);
        let clusters_to_read = (end_cluster-first_cluster+1) * self.get_bytes_in_cluster();
        let mut temp_buffer = vec![0u8;clusters_to_read];
        self.read_clusters(drive, file_pointer.get_file().get_fat_dir(), first_cluster, &mut temp_buffer);
        // println!("Temp Buffer: {}", unsafe { ::core::str::from_utf8_unchecked(&temp_buffer) });

        let first_index = file_pointer.get_current() % self.get_bytes_in_cluster();
        let last_index = first_index + buffer.len();
        buffer.clone_from_slice(&temp_buffer[first_index..last_index]);
        return Some(buffer.len());
    }
}