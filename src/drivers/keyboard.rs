use drivers::utils::inb;

pub fn get_scancode() -> u8 {
    let mut c: u8=0;
    loop {
        unsafe {
            if inb(0x60) != c {
                c = inb(0x60);
                if c > 0 {
                    return c;
                }
            }
        }
    }
}