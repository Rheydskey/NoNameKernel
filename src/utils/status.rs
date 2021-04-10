
use crate::Color;
use crate::ColorCode;
use crate::drivers::vga::buffer::Writer;
use core::fmt::Write;

pub struct Init<'a> {
    pub buffer: Writer,
    pub initname: &'a str,
}

impl<'a> Init<'a> {
    pub fn new(initname: &'static str) -> Self {
        let buffer = unsafe {crate::drivers::vga::render::BUFFER.get_mut().unwrap()};

        let position = buffer._get_position();

        buffer.new_line();

        let buffer = Writer::_from_position(position);

        Self { buffer, initname }
    }
    pub fn pending(&mut self) {
        self.buffer.color_code = ColorCode::new(Color::White, Color::Black);
        write!(self.buffer, "[ .. ] {}", &self.initname).expect("Error");
    }
    pub fn ok(&mut self) {
        self.buffer._clear_row(self.buffer.row_position);
        self.buffer._reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Green, Color::Black);
        write!(self.buffer, "[ OK ] {}", &self.initname).expect("Error");
    }
    pub fn _error(&mut self) {
        self.buffer._clear_row(self.buffer.row_position);
        self.buffer._reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Red, Color::Black);
        write!(self.buffer, "[ ERR ] {}", &self.initname).expect("Error");
    }
}
