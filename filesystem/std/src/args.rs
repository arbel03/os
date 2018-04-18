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