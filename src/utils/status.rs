use crate::drivers::vga::vga_color::Color;
use crate::drivers::vga::vga_color::ColorCode;
use core::fmt::Write;
use crate::drivers::vga::render::BUFFER;

pub enum Status {
    UNKNOW,
    ERROR,
    PENDING,
    OK,
}
pub struct Init<'a> {
    pub position: (usize, usize),
    pub status: Status,
    pub initname: &'a str,
}

impl<'a> Init<'a> {
    pub fn new(initname: &'static str) -> Self {
        let buffer = unsafe { crate::drivers::vga::render::BUFFER.get_mut().unwrap() };

        buffer.new_line();

        let position = buffer.get_position();

        Self {
            position,
            status: Status::UNKNOW,
            initname,
        }
    }
    pub fn pending(&mut self) {
        let buffer = if let Some(e) = unsafe {BUFFER.get_mut()} {e} else {return;};

        match self.status {
            Status::PENDING => return,
            _ => (),
        }

        buffer.color_code = ColorCode::new(Color::White, Color::Black);
        self.status = Status::PENDING;
        write!(buffer, "[ .. ] {}", &self.initname).expect("Error");
    }
    pub fn ok(&mut self) {
        let buffer = if let Some(e) = unsafe {BUFFER.get_mut()} {e} else {return;};

        match self.status {
            Status::OK => return,
            _ => (),
        }

        buffer.set_position(self.position);
        buffer.clear_line(self.position.1);
        buffer.color_code = ColorCode::new(Color::Green, Color::Black);
        self.status = Status::OK;
        write!(buffer, "[ OK ] {}", &self.initname).expect("Error");
    }
    pub fn error(&mut self) {
        let buffer = if let Some(e) = unsafe {BUFFER.get_mut()} {e} else {return;};

        match self.status {
            Status::ERROR => return,
            _ => (),
        }
        buffer.set_position(self.position);
        buffer.clear_line(self.position.1);
        buffer.color_code = ColorCode::new(Color::Red, Color::Black);
        self.status = Status::ERROR;
        write!(buffer, "[ ERR ] {}", &self.initname).expect("Error");
    }

    pub fn wait<F, E>(&mut self, callable: F) where F: FnOnce() -> Result<E, &'a str> {
        self.pending();
        let e = callable.call_once(());
        if let Ok(_) = e {
           self.ok();
        } else if let Err(msg) = e {
           self.error();
           panic!("{}", msg)
        }
    }
}
