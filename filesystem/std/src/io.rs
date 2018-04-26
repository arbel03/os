use core::fmt;
use alloc::String;

pub struct Terminal;

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        super::syscalls::printf(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use io::Terminal;
        let mut writer = Terminal;
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

pub fn read_string() -> String {
    use syscalls::{ getc,delc };
    use core::fmt::Write;

    let mut input_string = String::new();
    loop {
        let result = getc();
        let character = result as u8 as char;
        if character == '7' {
            continue;
        }
        if character == '\n' {
            print!("\n");
            return input_string;
        } else if result == 0xffffffff {
            if input_string.len() != 0 {
                delc();
                input_string.pop();
            }
        } else {
            print!("{}", character);
            input_string.write_char(character);
        }
    }
    return input_string;
}