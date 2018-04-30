#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate std;

use std::io;
use alloc::Vec;

#[no_mangle]
pub fn main(argc: usize, argv: *const *const u8) {
    let args = &unsafe { std::args::get_args(argc, argv) };
    loop {
        print!("{} $ ", args[0]);
        use alloc::string::ToString;
        let input = io::read_string();
        let command: Vec<&str> = input.trim().split(' ').collect();
        if command.len() > 0 {
            match command[0] {
                "echo" => println!("{}", input),
                _ => run_exec(command[0], &command[1..]),
            }
        } else {
            println!("Please enter a command.\nEnter 'Help' or '?' for more information.");
        }
    }
}

fn run_exec(path_name: &str, args: &[&str]) {
    std::syscalls::execv(path_name, args); 
}