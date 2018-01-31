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

#[repr(packed, C)]
#[derive(Debug, Copy, Clone)]
pub struct LongFileName {
    order: u8,
    name_first: [u16; 5],
    attributes: u8,
    long_entry_type: u8,
    checksum: u8,
    name_middle: [u16; 6],
    reserved: u16,
    name_final: [u16; 2],
}

impl LongFileName {
    pub fn get_name(&self) -> String {
        let mut buff = vec![0u16; 13];
        buff[..5].clone_from_slice(&self.name_first);
        buff[5..11].clone_from_slice(&self.name_middle);
        buff[11..].clone_from_slice(&self.name_final);

        // Replace null bytes and spaceholder values with spaces
        let mut last_index = buff.len();
        for (index, b) in buff.iter().enumerate() {
            // print!("{:#x} ", *b as u16);
            if *b == 0xffff || *b == 0 {
                last_index = index;
                break;
            }
        }

        // use alloc::string::ToString;
        // let name = String::from_utf16_lossy(&buff);
        return String::from_utf16_lossy(&buff[..last_index]);
    }
}

#[derive(Debug)]
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

    pub fn is_lfn(&self) -> bool {
        return self.fat_directory.is_lfn();
    }

    pub fn get_name(&self) -> String {
        use alloc::string::ToString;
        return self.name.to_string();
    }

    pub fn get_cluster(&self) -> u32 {
        return self.fat_directory.get_cluster();
    }

    pub fn is_folder(&self) -> bool {
        return self.fat_directory.attributes as u8 & FileAttributes::Directory as u8 == FileAttributes::Directory as u8;
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
    pub fn get_short_name(&self) -> String {
        use alloc::string::ToString;
        return String::from_utf8(self.name.to_vec()).expect("Invalid UTF-8.").trim().to_string();
    }

    pub fn get_cluster(&self) -> u32 {
        return (self.first_cluster_high as u32) << 16 | self.first_cluster_low as u32;
    }

    pub fn is_lfn(&self) -> bool {
        return self.attributes as u8 == FileAttributes::LongName as u8;
    }
}
