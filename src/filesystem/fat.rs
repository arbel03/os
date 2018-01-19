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