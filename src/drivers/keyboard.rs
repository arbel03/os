use drivers::utils::inb;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ScanCodeType {
    Digit(u8),
    Character(char),
    Shift,
    Backspace,
    Enter,
    Space,
    Quote,
}

pub struct ScanCode {
    pub released: bool,
    pub scan_code_type: ScanCodeType,
}

#[derive(PartialEq)]
pub enum ScanCodeError {
    BackspaceScancode,
    InvalidScancode,
}

impl ScanCode {
    pub fn new(scan_code_type: ScanCodeType) -> Self {
        ScanCode {
            released: false, 
            scan_code_type: scan_code_type,
        }
    }

    pub fn released(&self) -> Self {
        ScanCode {
            released: true,
            scan_code_type: self.scan_code_type.clone(),
        }
    }

    pub fn get_char(&self) -> Result<char, ScanCodeError> {
        let c = match self.scan_code_type {
            ScanCodeType::Digit(digit) => ('0' as u8 + digit) as char,
            ScanCodeType::Character(character) => {
                let character = character.to_string();
                let character = if unsafe { IS_UPPERCASE } { character.to_uppercase() } else { character };
                character.as_bytes()[0] as char
            },
            ScanCodeType::Enter => '\n',
            ScanCodeType::Quote => if unsafe { IS_UPPERCASE } { '\"' } else { '\'' },
            ScanCodeType::Space => ' ',
            ScanCodeType::Backspace => return Err(ScanCodeError::BackspaceScancode),
            _ => return Err(ScanCodeError::InvalidScancode),
        };
        Ok(c)
    }
}

use core::fmt;
use core::fmt::Write;
use alloc::string::ToString;
impl fmt::Display for ScanCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(ch) = self.get_char() {
            f.write_char(ch as char)
        } else {
            Ok(())
        }
    }
}

pub struct RawScanCode(u8);

impl RawScanCode {
    pub fn get_scancode(&self) -> Option<ScanCode> {
        let scancode = match self.0 {
            0x02 ... 0x0B => ScanCode::new(ScanCodeType::Digit(self.0 - 0x01)),
            0x0C => ScanCode::new(ScanCodeType::Digit(0)),
            0x0E => ScanCode::new(ScanCodeType::Backspace),
            0x10 => ScanCode::new(ScanCodeType::Character('q')),
            0x11 => ScanCode::new(ScanCodeType::Character('w')),
            0x12 => ScanCode::new(ScanCodeType::Character('e')),
            0x13 => ScanCode::new(ScanCodeType::Character('r')),
            0x14 => ScanCode::new(ScanCodeType::Character('t')),
            0x15 => ScanCode::new(ScanCodeType::Character('y')),
            0x16 => ScanCode::new(ScanCodeType::Character('u')),
            0x17 => ScanCode::new(ScanCodeType::Character('i')),
            0x18 => ScanCode::new(ScanCodeType::Character('o')),
            0x19 => ScanCode::new(ScanCodeType::Character('p')),
            0x1E => ScanCode::new(ScanCodeType::Character('a')),
            0x1F => ScanCode::new(ScanCodeType::Character('s')),
            0x20 => ScanCode::new(ScanCodeType::Character('d')),
            0x21 => ScanCode::new(ScanCodeType::Character('f')),
            0x22 => ScanCode::new(ScanCodeType::Character('g')),
            0x23 => ScanCode::new(ScanCodeType::Character('h')),
            0x24 => ScanCode::new(ScanCodeType::Character('j')),
            0x25 => ScanCode::new(ScanCodeType::Character('k')),
            0x26 => ScanCode::new(ScanCodeType::Character('l')),
            0x28 => ScanCode::new(ScanCodeType::Quote),
            0x2A => ScanCode::new(ScanCodeType::Shift),
            0x2B => ScanCode::new(ScanCodeType::Character('\\')),
            0x2C => ScanCode::new(ScanCodeType::Character('z')),
            0x2D => ScanCode::new(ScanCodeType::Character('x')),
            0x2E => ScanCode::new(ScanCodeType::Character('c')),
            0x2F => ScanCode::new(ScanCodeType::Character('v')),
            0x30 => ScanCode::new(ScanCodeType::Character('b')),
            0x31 => ScanCode::new(ScanCodeType::Character('n')),
            0x32 => ScanCode::new(ScanCodeType::Character('m')),
            0x33 => ScanCode::new(ScanCodeType::Character(',')),
            0x34 => ScanCode::new(ScanCodeType::Character('.')),
            0x35 => ScanCode::new(ScanCodeType::Character('/')),
            0x36 => ScanCode::new(ScanCodeType::Shift),
            0xAA => ScanCode::new(ScanCodeType::Shift).released(),
            0xB6 => ScanCode::new(ScanCodeType::Shift).released(),
            0x1C => ScanCode::new(ScanCodeType::Enter),
            0x39 => ScanCode::new(ScanCodeType::Space),
            _ => return None,
        };
        Some(scancode)
    }
}

static mut IS_UPPERCASE: bool = false;

pub fn set_uppercased(is_uppercased: bool) {
    unsafe {
        IS_UPPERCASE = is_uppercased;
    }
}

pub fn get_scancode() -> Option<ScanCode> {
    let scancode_value = read_scancode_value();
    let raw_scancode = RawScanCode(scancode_value);
    raw_scancode.get_scancode()
}

pub fn read_scancode_value() -> u8 {
    unsafe {
        while inb(0x64) & 1 != 1 {}
        inb(0x60)
    }
}

pub fn getc() -> usize {
    // set_uppercased(false);
    loop {
        if let Some(c) = get_scancode() {
            use drivers::keyboard::ScanCodeType;
            if ScanCodeType::Shift == c.scan_code_type {
                set_uppercased(!c.released);
            } else {
                match c.get_char() {
                    Ok(character) => return character as usize,
                    Err(scan_code_error) => {
                        if scan_code_error == ScanCodeError::BackspaceScancode {
                            return 0xffffffff;
                        }
                    }
                }
            }
        }
    }
}