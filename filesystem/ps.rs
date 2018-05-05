#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate std;

use std::io;

#[no_mangle]
pub unsafe fn main(_argc: usize, _argv: *const *const u8) {
    let mut occupied_size = 0;
    for i in 0..100 {
        if let Some(proc_info) = std::syscalls::proc_info(i) {
            println!("Process number {}", i);
            println!("\tProcess start: {:#x}", proc_info.process_base);
            println!("\tProcess end: {:#x}", proc_info.process_total_size);
            occupied_size += proc_info.process_total_size as usize;
        } else {
            break;
        }
    }
    let total_size = std::syscalls::get_proccess_area_size();
    let free = total_size - occupied_size;
    println!("\n===== Process area info =====");
    println!("Total: {} bytes.", total_size);
    println!("Free: {} bytes.", free);
    println!("Occupied: {} bytes.", occupied_size);
}