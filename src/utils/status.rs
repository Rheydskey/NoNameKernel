use crate::lib::vga::Writer;
use crate::Color;
use crate::ColorCode;
use core::fmt::Write;

pub struct Init<'a> {
    pub initname: &'a str,
    pub buffer: Writer,
}

impl<'a> Init<'a> {
    pub fn new(position: (usize, usize), initname: &'static str) -> Self {
        let buffer = Writer::from_position(position);
        Self { initname, buffer }
    }
    pub fn pending(&mut self) {
        self.buffer.color_code = ColorCode::new(Color::White, Color::Black);
        write!(self.buffer, "[ .. ] {}", &self.initname).expect("Error");
    }
    pub fn ok(&mut self) {
        self.buffer._clear_row(self.buffer.row_position);
        self.buffer.reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Green, Color::Black);
        write!(self.buffer, "[ OK ] {}", &self.initname).expect("Error");
    }
    pub fn error(&mut self) {
        self.buffer._clear_row(self.buffer.row_position);
        self.buffer.reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Red, Color::Black);
        write!(self.buffer, "[ ERR ] {}", &self.initname).expect("Error");
    }
}
