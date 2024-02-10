use core::{ffi::c_char, fmt::Write};

use crate::bindings::c_bindings::LiquidCrystal_I2C;

pub struct LiquidCrystal {
    lc: LiquidCrystal_I2C,
    cols: u8,
    rows: u8,
}

const BUFFER_SIZE: usize = 32;

pub enum LiquidCrystalError {
    BufferOverflow,
    InvalidSize(u8, u8),
}

pub enum Backlight {
    On,
    Off,
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

    pub fn backlight(&mut self, b: Backlight) {
        match b {
            Backlight::On => unsafe { self.lc.backlight() },
            Backlight::Off => unsafe { self.lc.noBacklight() },
        }
    }

    pub fn clear(&mut self) {
        unsafe { self.lc.clear() }
    }

    pub fn set_cursor(&mut self, x: u8, y: u8) -> Result<(), LiquidCrystalError> {
        if x > self.cols {
            return Err(LiquidCrystalError::InvalidSize(x, self.cols));
        }

        if y > self.rows {
            return Err(LiquidCrystalError::InvalidSize(y, self.rows));
        }

        unsafe { self.lc.setCursor(x, y) }
        Ok(())
    }

    pub fn print(&mut self, s: &str) -> Result<(), LiquidCrystalError> {
        let bytes = s.as_bytes();

        if bytes.len() >= BUFFER_SIZE {
            return Err(LiquidCrystalError::BufferOverflow);
        }
        if bytes.len() >= self.cols as usize {
            return Err(LiquidCrystalError::InvalidSize(
                bytes.len() as u8,
                self.cols,
            ));
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

impl Write for LiquidCrystal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.print(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(core::fmt::Error),
        }
    }
}
