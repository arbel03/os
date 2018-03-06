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

pub fn syscall(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    println!("Syscall received. ({}, {}, {}, {}, {}, {})", a, b, c, d, e, f);
    let x = match a & SYSCALL_CLASS {
        FILESYSTEM_CLASS => { 
            let fd = b;
            match a & SYSCALL_METHOD {
                CALL_OPEN => open(to_str(b, c)),
                _ => Err(0xFFFFFFFF),
            }
        },
        _ => Err(0xFFFFFFFF),
    };
    if x.is_err() {
        return x.unwrap_err() as usize;
    }
    return x.unwrap() as usize;
}