#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate std;

use std::io;

#[no_mangle]
pub unsafe fn main(argc: usize, argv: *const *const u8) {
    let args = &std::args::get_args(argc, argv);
    if args.len() == 1 {
        println!("Usage:\n\t{0} DIRECTORY_NAME\n\t\tDIRECTORY_NAME can be either a folder or a file.\n\t\t This command prints information about the given directory.", args[0]);
        return;
    }
    let stat = std::syscalls::stat(args[1], 0);
    println!("Printing information for the {}: \"{}\"", if stat.is_folder { "folder" } else { "file" }, args[1]);
    println!("Length of entry name: {}", stat.directory_name_length);
    if stat.is_folder {
        println!("Number of child folders/files: {}", stat.child_nodes_count);
    } else {
        println!("File size: {}", stat.directory_size);
    }
}