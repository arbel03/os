pub mod fs;

use self::fs::*;
use core::slice;
use core::str;
use super::task::CURRENT_PROCESS;

pub fn to_str<'a>(ptr: usize, size: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr as *const u8, size);
        str::from_utf8(slice).unwrap()
    }
}

const SYS_FOPEN: usize = 0x1;
const SYS_PRINT: usize = 0x2;
const SYS_READ: usize = 0x03;
const SYS_FILESZ: usize = 0x04;
const UNDEFINED_SYSCALL: usize = 0xff;

#[allow(unused_variables)]
pub unsafe fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    let current_process = CURRENT_PROCESS.as_ref().unwrap();
    match a {
        SYS_FOPEN => {         
            let ptr = current_process.translate_virtual_to_physical_address(b as *const u8);
            open(to_str(ptr as usize, c)) 
        },
        SYS_PRINT => {
            let ptr = current_process.translate_virtual_to_physical_address(b as *const u8);
            let string = to_str(ptr as usize, c);
            print!("{}", string);
            0
        },
        SYS_READ => {
            let ptr = current_process.translate_virtual_to_physical_address(c as *const u8);
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, d);
            read(b, slice)
        },
        SYS_FILESZ => {
            file_size(b)
        },
        _ => UNDEFINED_SYSCALL
    }
}