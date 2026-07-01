#![no_std]
#![no_main]

mod vga;

///---Constants---///
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

///---Structures-And-Enums---///
pub struct Writer {
    columnPos: usize,
    colorCode: ColorCode,
    buffer: &'static mut Buffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    asciiCharacter: u8,
    colorCode: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

///---Implimentantions---///
impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

impl Writer {
    pub fn writeByte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newLine(),
            byte => {
                if self.columnPos >= BUFFER_WIDTH {
                    self.newLine();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.columnPos;

                let colorCode = self.colorCode;
                self.buffer.chars[row][col] = ScreenChar {
                    asciiCharacter: byte,
                    colorCode,
                };
                self.columnPos += 1;
            }
        }
    }

    fn newLine(&mut self) {/* TODO */}

    pub fn kprint(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.writeByte(byte),
                _ => self.writeByte(0xfe),
            }
        }
    }
}
