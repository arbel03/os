use core::fmt;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        use core::fmt::Write;
        match byte {
            b'\t' => {
                self.write_str("    ");
            },
            b'\n' => {
                self.new_line();   
                
            },
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();   
                }
                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                };
                self.column_position += 1;
            }
        }
        self.update_cursor();
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    }

    fn new_line(&mut self) {
        if self.row_position == BUFFER_HEIGHT-1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let buffer = self.buffer();
                    let character = buffer.chars[row][col];
                    buffer.chars[row-1][col] = character;
                }
            }
            self.clear_row(BUFFER_HEIGHT-1);
        }

        use core::cmp::min;
        self.row_position = min(self.row_position+1, BUFFER_HEIGHT-1);
        self.column_position = 0;

        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col] = blank;
        }
    }

    fn update_cursor(&self) {
        ::drivers::cursor::Cursor.update_location(self.row_position, self.column_position);
    }

    pub fn delete_char(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let mut col = self.column_position;
        let mut row = self.row_position;
        if row == 0 && col == 0 {
            return;
        }
        if col == 0 {
            row -= 1;
            col = BUFFER_WIDTH-1;
        } else {
            col -= 1;
        }
        self.column_position = col;
        self.row_position = row;
        self.buffer().chars[row][col] = blank;
        self.update_cursor();
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static mut WRITER: Writer = Writer {
    column_position: 0,
    row_position: 0,
    color_code: ColorCode::new(Color::LightGray, Color::Black),
};

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let writer = unsafe { &mut $crate::vga_buffer::WRITER };
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[allow(unused_macros)]
macro_rules! hex_dump {
    ($value: expr) => {
       for (i, n) in $value.iter().enumerate() {
            print!("{:02x} ", *n);
            if (i+1) % 16 == 0 {
                print!("\n");
            } else if (i+1) % 8 == 0 {
                print!("  ");
            }
        }
        println!("");
    };
}

pub fn clear_screen() {
    unsafe {
        for row in 0..BUFFER_HEIGHT {
            WRITER.clear_row(row);
        }
        WRITER.column_position = 0;
        WRITER.row_position = 0;
    }
}