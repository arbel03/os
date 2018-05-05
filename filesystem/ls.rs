#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate std;

use std::io;
use std::syscalls::Stat;
use alloc::string::ToString;
use alloc::string::String;

#[no_mangle]
pub fn main(argc: usize, argv: *const *const u8) {
    let args = &unsafe { std::args::get_args(argc, argv) };
    if args.len() == 1 {
        println!("Usage:\n\t{0} -r\n\t\tPrints the filesystem recursively.\n\t{0} FOLDER_NAME\n\t\tPrints the folder contents.", args[0]);
        return;
    }
    let second_param = args[1].trim().to_string().to_lowercase();
    if second_param == "-r" {
        println!("Printing filesystem recursively.");
        recursive_ls(".", 0);
    } else {
        let stat = std::syscalls::stat(&second_param, 0);
        for i in 0..stat.child_nodes_count as usize {
            println!("{}", read_name(&second_param, i).1);
        }
    }
}

pub fn recursive_ls(path: &str, level: usize) {
    let current_status = std::syscalls::stat(path, 0);
    for child in 0..current_status.child_nodes_count as usize {
        let (stat, name) = read_name(path, child);
        println!("{}{}", "\t".repeat(level) ,name);
        if stat.is_folder && name != "." && name != ".." {
            let mut current_path = if path == "." {
                String::new()
            } else {
                let mut new_path = path.to_string();
                new_path.push_str("/");
                new_path
            };
            current_path.push_str(&name);
            recursive_ls(&current_path, level + 1);
        }
    }
}

pub fn read_name(parent_directory: &str, child_node: usize) -> (Stat, String) {
    let child_status = std::syscalls::stat(parent_directory, child_node+1);
    let mut name = vec![0u8;child_status.directory_name_length as usize];
    unsafe {
        std::syscalls::read_name(parent_directory, &mut name, child_node);
        let string = ::core::str::from_utf8_unchecked(&name);
        (child_status, string.to_string())
    }
}