use crate::lib::vga_color::{Color, ColorCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 20;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub row_position: usize,
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn default() -> Self {
        Writer {
            row_position: 0,
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;
    }
}
use core::fmt::{Write, Result as WriteResult};

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> WriteResult {
        self.write_string(s);
        Ok(())
    }
}
