#![no_std]
#![feature(fn_traits)]
#![feature(format_args_nl)]

#[macro_use]
extern crate nmk_drivers;

pub enum Status {
    Unknow,
    Error,
    Pending,
    Ok,
}
pub struct Init<'a> {
    pub status: Status,
    pub initname: &'a str,
}

impl<'a> Init<'a> {
    pub fn new(initname: &'static str) -> Self {
        Self {
            status: Status::Unknow,
            initname,
        }
    }
    pub fn pending(&mut self) {
        if matches!(self.status, Status::Pending) {
            return;
        }

        self.status = Status::Pending;
        println!("[ .. ] {}", &self.initname);
    }
    pub fn ok(&mut self) {
        if matches!(self.status, Status::Ok) {
            return;
        }

        println!("[ OK ] {}", &self.initname);
    }
    pub fn error(&mut self) {
        if matches!(self.status, Status::Error) {
            return;
        };

        self.status = Status::Error;
        println!("[ ERR ] {}", &self.initname);
    }

    pub fn wait<F, E>(&mut self, callable: F)
    where
        F: FnOnce() -> Result<E, &'a str>,
    {
        println!("Init: {}", self.initname);
        self.pending();
        let e = callable.call_once(());
        if e.is_ok() {
            println!("OK : {}", self.initname);
            self.ok();
        } else if let Err(msg) = e {
            println!("ERROR : {}", self.initname);
            self.error();
            panic!("{}", msg)
        }
    }
}
