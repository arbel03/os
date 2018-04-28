mod fs;
mod task;

pub use self::fs::*;
pub use self::task::*;
use core::{ slice, str };
use alloc::Vec;

pub fn to_str<'a>(ptr: usize, size: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr as *const u8, size);
        if let Ok(string) = str::from_utf8(slice) {
            return string;
        }
        return "Invalid String.";
    }
}

const SYS_FOPEN: usize = 0x1;
const SYS_PRINT: usize = 0x2;
const SYS_READ: usize = 0x03;
const SYS_FILESZ: usize = 0x04;
const IO_GETC: usize = 0x05;
const IO_DELC: usize = 0x06;
const SYS_EXECV: usize = 0x07;
const UNDEFINED_SYSCALL: usize = 0xff;

#[allow(unused_variables)]
pub unsafe fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    let current_process = ::task::get_current_process();
    match a {
        SYS_FOPEN => {         
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            open(to_str(ptr as usize, c)) 
        },
        SYS_PRINT => {
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let string = to_str(ptr as usize, c);
            print!("{}", string);
            0
        },
        SYS_READ => {
            let ptr = current_process.get_load_information().translate_virtual_to_physical_address(c as *const u8);
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, d);
            read(b, slice)
        },
        IO_GETC => {
            ::drivers::keyboard::getc() as usize
        },
        IO_DELC => {
            ::vga_buffer::WRITER.delete_char();
            0
        },
        SYS_FILESZ => {
            file_size(b)
        },
        SYS_EXECV => {
            unsafe fn read_args<'a>(argv: *const *const u8, argc: usize, process: &::task::process::Process) -> Vec<&'a str> {
                pub unsafe fn terminated_string<'a>(start: *const u8) -> &'a str {
                    use core::{ str, slice, ptr };

                    let mut length: isize = 0;
                    loop {
                        let current = start.offset(length);
                        if ptr::read(current) == 0u8 {
                            break;
                        }
                        length += 1;
                    }
                    return str::from_utf8_unchecked(slice::from_raw_parts(start, length as usize));
                }

                let mut args: Vec<&str> = Vec::with_capacity(argc);
                let ptr_slice = slice::from_raw_parts(argv, argc);
                for (index, ptr) in ptr_slice.iter().enumerate() {
                    let physical_ptr = process.get_load_information().translate_virtual_to_physical_address(ptr.clone());
                    args.push(terminated_string(physical_ptr));
                }
                return args;
            }
            
            let name_ptr = current_process.get_load_information().translate_virtual_to_physical_address(b as *const u8);
            let args_ptr = current_process.get_load_information().translate_virtual_to_physical_address(d as *const u8) as *const *const u8;
            execv(to_str(name_ptr as usize, c), &read_args(args_ptr, e, &current_process))
        }
        _ => UNDEFINED_SYSCALL
    }
}