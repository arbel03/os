pub unsafe fn terminated_string<'a>(start: *const u8) -> &'a str {
    use core::{ str, slice, ptr };

    let mut length: isize = 0;
    loop {
        let current = start.offset(length);
        // println!("{:?} - {}", current, ptr::read(current));
        if ptr::read(current) == 0u8 {
            break;
        }
        length += 1;
    }
    return str::from_utf8_unchecked(slice::from_raw_parts(start, length as usize));
}

use alloc::Vec;
pub unsafe fn get_args<'a>(argc: usize, argv: *const *const u8) -> Vec<&'a str> {
    use core::slice;
    let str_pointer_slice = slice::from_raw_parts(argv, argc);
    let mut str_array: Vec<&str> = Vec::new();
    for str_pointer in str_pointer_slice.iter() {
        let string = terminated_string(str_pointer.clone());
        // println!("{} at {:?}", string, str_pointer);
        str_array.push(string);
    }
    return str_array;
}