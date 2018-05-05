use filesystem::descriptor::File;
use alloc::string::String;

#[repr(u8)]
#[allow(dead_code)]
pub enum FileAttributes {
    ReadOnly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    VolumeId = 0x08,
    Directory = 0x10,
    Archive = 0x20,
    LongName = FileAttributes::ReadOnly as u8 | 
    FileAttributes::Hidden as u8 | 
    FileAttributes::System as u8 | 
    FileAttributes::VolumeId as u8,
} 

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    fat_directory: FatDirectory,
}

impl Directory {
    pub fn new(name: String, directory: FatDirectory) -> Self {
        Directory {
            name: name,
            fat_directory: directory,
        }
    }

    pub fn get_fat_dir(&self) -> &FatDirectory {
        &self.fat_directory
    }
}

impl File for Directory {
    fn get_name(&self) -> String {
        use alloc::string::ToString;
        return self.name.to_string();
    }

    fn get_size(&self) -> usize {
        self.fat_directory.file_size as usize
    }
}

#[repr(packed, C)]
#[derive(Debug, Copy, Clone)]
pub struct FatDirectory {
    pub name: [u8; 11],
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

impl FatDirectory {
    pub fn with_cluster(cluster: u32) -> Self {
        FatDirectory {
            name: ['.' as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            attributes: FileAttributes::Directory as u8,
            flags_nt: 0,
            creation_time_precise: 0,
            creation_time: 0,
            creation_date: 0,
            last_accessed: 0,
            first_cluster_high: (cluster >> 16) as u16,
            last_modified_time: 0,
            last_modified_date: 0,
            first_cluster_low: cluster as u16,
            file_size: 0,
        }
    }

    pub fn get_short_name(&self) -> String {
        use alloc::string::ToString;
        String::from_utf8(self.name.to_vec()).expect("Invalid UTF-8.").trim().to_string()
    }

    pub fn get_cluster(&self) -> u32 {
        (self.first_cluster_high as u32) << 16 | self.first_cluster_low as u32
    }

    // pub fn get_size(&self) -> usize {
    //     self.file_size as usize
    // }

    pub fn is_lfn(&self) -> bool {
        self.attributes as u8 == FileAttributes::LongName as u8
    }

    pub fn is_folder(&self) -> bool {
        self.attributes as u8 & FileAttributes::Directory as u8 == FileAttributes::Directory as u8
    }

    pub unsafe fn get_long_name(&self) -> String {
        use core::slice;
        // Slice of 32 bytes
        let bytes = slice::from_raw_parts(self as *const FatDirectory as *const u8, 32);
        let name_first = slice::from_raw_parts(&bytes[1] as *const u8 as *const u16, 5);
        let name_middle = slice::from_raw_parts(&bytes[14] as *const u8 as *const u16, 6);
        let name_final = slice::from_raw_parts(&bytes[28] as *const u8 as *const u16, 2);

        let mut buff = vec![0u16; 13];
        buff[..5].clone_from_slice(name_first);
        buff[5..11].clone_from_slice(name_middle);
        buff[11..].clone_from_slice(name_final);

        let mut last_index = buff.len();
        for (index, b) in buff.iter().enumerate() {
            if *b == 0xffff || *b == 0 {
                last_index = index;
                break;
            }
        }

        String::from_utf16_lossy(&buff[..last_index])
    }
}
