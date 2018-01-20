use super::Filesystem;
use super::FilePointer;
use super::disk::Disk;

#[repr(packed, C)]
#[derive(Debug)]
pub struct Bpb {
    skip_code: [u8; 3],
    oem_identifier: [u8;8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    number_fat: u8,
    directory_entries: u16,
    total_sectors: u16,
    media_descriptor_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    number_heads: u16,
    hidden_sectors: u32,
    large_amount_of_sector: u32,
}

#[repr(packed, C)]
#[derive(Debug)]
pub struct Ebpb {
    bpb: Bpb,
    sectors_per_fat: u32,
    flags: u16,
    version_number: u16,
    root_dir_cluster: u32,
    fsinfo_sector: u16,
    backup_mbr_sector: u16,
    reserved: [u8; 12],
    drive_number: u8,
    flags_nt: u8,
    signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    system_identifier: [u8; 8]
}

#[repr(packed, C)]
pub struct FatFile {
    name: [u8; 11],
    attributes: u8,
    flags_nt: u8,
    creation_time_precise: u8,
    creation_time: u16,
    creation_date: u16,
    last_accessed: u16,
    first_cluster_high: u16,
    last_modified_time: u16,
    last_modified_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

enum FatEntry {
    Valid(u32),
    End,
    BadBlock,
}

// Fat32 implementaion
pub struct Fat32<'a> {
    ebpb: &'a Ebpb,
}

impl <'a> Fat32<'a> {
    pub const fn new(ebpb: &'a Ebpb) -> Self {
        Fat32 {
            ebpb: ebpb,
        }
    }

    fn get_entry(&self, drive: &Disk, index: u32) -> FatEntry {
        let mut fat_table = vec![0u8; 512];
        let fat_offset = index*4;
        let fat_sector = self.ebpb.bpb.reserved_sectors as u32 + (fat_offset / 512);
        let ent_offset = fat_offset % 512;
        let table_value = unsafe {
            drive.read(fat_sector as u64, &mut fat_table).expect("Unknown error.");
            let table_reference = &fat_table[ent_offset as usize] as *const u8 as *const u32;
            *table_reference & 0x0FFFFFFF
        };

        hex_dump!(fat_table);

        if table_value >= 0x0FFFFFF8 {
            return FatEntry::End;
        } else if table_value == 0x0FFFFFF7 {
            return FatEntry::BadBlock;
        } else {
            return FatEntry::Valid(table_value);
        }
    }
}

impl <'a> Filesystem for Fat32<'a> {
    type FileType = FatFile;

    fn open_file(&self, drive: &Disk, file_name: &str) -> FilePointer<Self::FileType> {
        let mut current_cluster: u32 = self.ebpb.root_dir_cluster;
        loop {
            println!("current_cluster: {}", current_cluster);
            let value = self.get_entry(drive, current_cluster);
            match value {
                FatEntry::Valid(next) => current_cluster = next,
                _ => break,
            }
        }

        FilePointer {
            current_position: 0,
            file: FatFile {
                name: [0;11],
                attributes: 1,
                flags_nt: 2,
                creation_time_precise: 3,
                creation_time: 4,
                creation_date: 5,
                last_accessed: 6,
                first_cluster_high: 7,
                last_modified_time: 8,
                last_modified_date: 9,
                first_cluster_low: 0,
                file_size: 11,
            }
        }
    }

    fn read_file(&self, drive: &Disk, descriptor: FilePointer<Self::FileType>, buffer: &[u8], count: u64) {

    }
}