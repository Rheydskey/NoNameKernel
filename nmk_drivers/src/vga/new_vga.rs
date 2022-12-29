use core::fmt::Debug;

#[repr(C)]
pub struct Vga {
    height: u16,
    width: u16,
    bpp: u16,
    pitch: u16,
    addr: u64,
}

impl Vga {
    pub const fn new(addr: u64, height: u16, width: u16, bpp: u16, pitch: u16) -> Self {
        let vga = Self {
            height,
            width,
            bpp,
            pitch,
            addr,
        };
        let fb =
            unsafe { core::slice::from_raw_parts_mut(vga.addr as *mut _, vga.size() as usize) };

        return vga;
    }

    pub fn write(&mut self, x: u64, y: u64) {
        let fb_x = (self.bpp as u64 / 32) * x;

        let fb_y = (self.pitch as u64 / 8) * y;

        let wh = fb_x + fb_y;

        self.change_value(wh as usize, 255 & 255, 255, 255)
    }

    pub fn change_value(&mut self, whe: usize, r: u8, g: u8, b: u8) {
        fb[whe] = (r & g & b) as u64;
    }

    pub fn size(&self) -> u64 {
        self.height as u64 * self.width as u64 * (self.bpp as u64 / 8)
    }

    pub fn height(&self) -> u64 {
        self.height as u64 + self.height as u64 * (self.bpp as u64 / 32)
    }
    pub fn width(&self) -> u64 {
        self.width as u64
    }
}

impl Debug for Vga {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vga")
            .field("height", &self.height)
            .field("width", &self.width)
            .field("bpp", &self.bpp)
            .finish()
    }
}
