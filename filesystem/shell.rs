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
                "echo" => {
                    let command = input.trim();
                    if command.len() >= 5 {
                        println!("{}", &command[5..]);
                    }
                },
                "help" => {
                    println!("Available commands:\n\t-echo\n\t-help");
                },
                _ => { 
                    let result = run_exec(command[0], &command[1..]);
                    if result == 0xffffffff {
                        println!("Command \"{}\" wasn't found.", command[0]);
                    }
                },
            }
        } else {
            println!("Please enter a command.\nEnter 'Help' or '?' for more information.");
        }
    }
}

fn run_exec(path_name: &str, args: &[&str]) -> usize {
    std::syscalls::execv(path_name, args)
}