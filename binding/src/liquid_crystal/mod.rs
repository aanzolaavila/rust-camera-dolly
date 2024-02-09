use core::ffi::c_char;

use crate::bindings::c_bindings::LiquidCrystal_I2C;

use super::bindings;

pub struct LiquidCrystal {
    lc: LiquidCrystal_I2C,
    cols: u8,
    rows: u8,
}

const BUFFER_SIZE: usize = 32;

pub enum LiquidCrystalError {
    BufferOverflow,
    InvalidSize(u8),
}

impl LiquidCrystal {
    pub fn new(addr: u8, cols: u8, rows: u8) -> Self {
        let mut lc = unsafe { LiquidCrystal_I2C::new(addr, cols, rows) };
        unsafe {
            lc.begin(cols, rows, 0);
            lc.init();
        }

        Self { lc, cols, rows }
    }

    pub fn backlight(&mut self) {
        unsafe { self.lc.backlight() }
    }

    pub fn clear(&mut self) {
        unsafe { self.lc.clear() }
    }

    pub fn set_cursor(&mut self, x: u8, y: u8) {
        unsafe { self.lc.setCursor(x, y) }
    }

    pub fn print(&mut self, s: &str) -> Result<(), LiquidCrystalError> {
        let bytes = s.as_bytes();

        if bytes.len() >= BUFFER_SIZE {
            return Err(LiquidCrystalError::BufferOverflow);
        }
        if bytes.len() >= self.cols as usize {
            return Err(LiquidCrystalError::InvalidSize(bytes.len() as u8));
        }

        let mut buffer: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE];

        for (i, &byte) in bytes.iter().enumerate() {
            buffer[i] = byte as c_char;
        }

        // null terminator
        buffer[bytes.len()] = 0;

        unsafe {
            self.lc.printstr(buffer.as_ptr());
        }

        Ok(())
    }
}
