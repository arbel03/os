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
                    println!("Available commands:\n\t-echo\n\t-help\n\t-printheap\n\t-bin/ls\n\t-bin/cat\n\t-bin/stat\n\t-bin/ps");
                },
                "printheap" => {
                    println!("Enter heap read start and read size.");
                    println!("Heap size is {}. Don't go out of bounds.", std::HEAP_SIZE);
                    let start = input_number("Enter read start");
                    let size = input_number("Enter read size");
                    unsafe { print_heap(start, size); }
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

pub fn input_number(prompt: &str) -> usize {
    loop {
        print!("{}: ", prompt);
        if let Ok(number) = io::read_string().parse() {
            return number;
        } else {
            println!("Please enter a valid number.");
        }
    }
}

pub unsafe fn print_heap(start: usize, size: usize) {
    use std::HEAP_SIZE;
    use std::HEAP_AREA;
    // use bitmap_allocator::CellState;
    // let allocator = &HEAP;
    // println!("Printing bitmap:");
    // let bitmap_size = allocator.get_block_count();
    // for index in 0..bitmap_size {
    //     let block = allocator.get_cell(index).clone();
    //     let block_string = match block {
    //         CellState::Free => "_",
    //         CellState::Boundary => "<",
    //         CellState::Allocated => "=",
    //     };
    //     print!("{}", block_string);
    // }
    // print!("\n");
    if start >= HEAP_SIZE {
        println!("Can't start reading heap from outside of bounds.");
        println!("Please choose a lower value than {}.", HEAP_SIZE);
    } else if start + size > HEAP_SIZE {
        println!("End of read is at {}", start+size);
        println!("Maximum bytes that can be read is {}", HEAP_SIZE - start);
    } else {
        let ptr = (&HEAP_AREA).as_ptr();
        let len = HEAP_SIZE;
        let string = core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len));
        println!("{}", &string[start..start+size]);
    }
}