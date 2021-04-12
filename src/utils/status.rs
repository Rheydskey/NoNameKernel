use crate::drivers::vga::buffer::Writer;
use crate::Color;
use crate::ColorCode;
use core::fmt::Write;

pub enum Status {
    UNKNOW,
    ERROR,
    PENDING,
    OK,
}
pub struct Init<'a> {
    pub buffer: Writer,
    pub status: Status,
    pub initname: &'a str,
}

impl<'a> Init<'a> {
    pub fn new(initname: &'static str) -> Self {
        let buffer = unsafe { crate::drivers::vga::render::BUFFER.get_mut().unwrap() };

        let position = buffer._get_position();

        buffer.new_line();

        let buffer = Writer::_from_position(position);

        Self {
            buffer,
            status: Status::UNKNOW,
            initname,
        }
    }
    pub fn pending(&mut self) {
        match self.status {
            Status::PENDING => return,
            _ => (),
        }
        self.buffer.color_code = ColorCode::new(Color::White, Color::Black);
        self.status = Status::PENDING;
        write!(self.buffer, "[ .. ] {}", &self.initname).expect("Error");
    }
    pub fn ok(&mut self) {
        match self.status {
            Status::OK => return,
            _ => (),
        }
        self.buffer.clear_row(self.buffer.row_position);
        self.buffer.reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Green, Color::Black);
        self.status = Status::OK;
        write!(self.buffer, "[ OK ] {}", &self.initname).expect("Error");
    }
    pub fn _error(&mut self) {
        match self.status {
            Status::ERROR => return,
            _ => (),
        }
        self.buffer.clear_row(self.buffer.row_position);
        self.buffer.reset_cursor();
        self.buffer.color_code = ColorCode::new(Color::Red, Color::Black);
        self.status = Status::ERROR;
        write!(self.buffer, "[ ERR ] {}", &self.initname).expect("Error");
    }
}
