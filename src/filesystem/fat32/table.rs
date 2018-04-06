use super::{ Disk, Fat32 };

#[repr(packed, C)]
#[derive(Debug, Clone, Copy)]
pub struct Bpb {
    pub skip_code: [u8; 3],
    pub oem_identifier: [u8;8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors_count: u16,
    pub table_count: u8,
    pub root_entry_count: u16,
    pub total_sectors: u16,
    pub media_descriptor_type: u8,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16,
    pub head_size_count: u16,
    pub hidden_sectors_count: u32,
    pub total_sectors_large: u32,
}

#[repr(packed, C)]
#[derive(Debug, Clone, Copy)]
pub struct Ebpb {
    pub bpb: Bpb,
    pub sectors_per_fat: u32,
    pub flags: u16,
    pub version_number: u16,
    pub root_dir_cluster: u32,
    pub fsinfo_sector: u16,
    pub backup_mbr_sector: u16,
    pub reserved: [u8; 12],
    pub drive_number: u8,
    pub flags_nt: u8,
    pub signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub system_identifier: [u8; 8]
} 

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum FatEntry {
    Node(Cluster),
    End,
    BadBlock,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cluster(pub usize);

pub struct ClusterChain<'a> {
    current_entry: FatEntry,
    fat: &'a Fat32,
    drive: &'a Disk,
}

impl <'a> ClusterChain<'a> {
    pub const fn new(cluster: Cluster, fat: &'a Fat32, drive: &'a Disk) -> Self {
        ClusterChain {
            current_entry: FatEntry::Node(cluster),
            fat: fat,
            drive: drive,
        }
    }

    fn read_entry(&self, current: Cluster) -> FatEntry {
        let sector_size = self.fat.ebpb.bpb.bytes_per_sector as usize;
        // Buffer to hold sector data
        let mut fat_table = vec![0u8; sector_size];
        // Each entry is 4 bytes
        let fat_offset = current.0 * 4;
        // Finding the sector we need to access, this works because integers division produces an integer
        let fat_sector = self.fat.ebpb.bpb.reserved_sectors_count as usize + (fat_offset / sector_size);
        // Offset within the chosen sector
        let ent_offset = fat_offset % sector_size;
        // Loading sector into fat_table vector and getting the table value
        let table_value = unsafe {
            self.drive.read(fat_sector as u64, &mut fat_table).expect("Unknown error.");
            let table_reference = &fat_table[ent_offset as usize] as *const u8 as *const u32;
            *table_reference & 0x0FFFFFFF
        };

        // Returning value
        if table_value >= 0x0FFFFFF8 {
            return FatEntry::End;
        } else if table_value == 0x0FFFFFF7 {
            return FatEntry::BadBlock;
        } else {
            // FatEntry pointing to the next index in the table
            return FatEntry::Node(Cluster(table_value as usize));
        }
    }
}

impl <'a> Iterator for ClusterChain <'a> {
    type Item = Cluster;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry == FatEntry::End || self.current_entry == FatEntry::BadBlock {
            return None;
        }

        let current_index = match self.current_entry {
            FatEntry::Node(current_cluster) => {
                current_cluster
            },
            _ => panic!("Shouldn't arrive here."),
        };
        self.current_entry = self.read_entry(current_index);

        return Some(current_index);
    }
}