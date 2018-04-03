pub mod fs;
mod io;

pub use self::io::*;
use self::fs::*;
use core::slice;
use core::str;

pub fn to_str<'a>(ptr: usize, size: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr as *const u8, size);
        str::from_utf8(slice).unwrap()
    }
}

const SYSCALL_CLASS: usize = 0x0F;
const SYSCALL_METHOD: usize = 0xF0;

const FILESYSTEM_CLASS: usize = 0x01;
const CALL_OPEN: usize = 0x10;

#[allow(unused_variables)]
pub unsafe fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    let x = match a & SYSCALL_CLASS {
        FILESYSTEM_CLASS => { 
            let fd = b;
            match a & SYSCALL_METHOD {
                // TODO- replace 0x15b000 with the base of the current process data segment.
                CALL_OPEN => open(to_str(b+0x15b000, c)),
                _ => 0xFFFFFFFF,
            }
        },
        _ => 0xFFFFFFFF,
    };
    return x;
}