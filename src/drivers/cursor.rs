use super::utils::outb;

pub struct Cursor;

impl Cursor {
    pub fn update_location(&self, row: usize, column: usize) {
        use vga_buffer::BUFFER_WIDTH;
        let pos = row * BUFFER_WIDTH + column;

        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
    }
}