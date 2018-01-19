pub mod disk;
pub mod fat;

use core::mem::transmute;
use drivers::ata::Ata;
use self::disk::Disk;
use self::fat::*;

pub fn detect() {
    let mut x = [0u8;512];
    match Ata::PRIMARY.read(0, &mut x) {
        Ok(read_amount) => 
        println!("{} sectors were read.", read_amount),
        Err(err_msg) => println!("{}", err_msg),
    }

    // let bpb = mem::transmute<&[u8],&Bpb>(&x.as_ptr()); 
    // println!("{:?}", slice);
    // loop {};
}