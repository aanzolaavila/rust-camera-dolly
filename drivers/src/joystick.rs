use super::arduino::{
    io::{AnalogRead, DigitalRead},
    pins::{analog_pin::AnalogInput, digital_pin::DigitalInput},
};

pub struct Joystick {
    x_pin: AnalogInput,
    y_pin: AnalogInput,
    switch_pin: DigitalInput,
    initial_pos: (u16, u16),
}

impl Joystick {
    pub fn new(x_pin: AnalogInput, y_pin: AnalogInput, switch_pin: DigitalInput) -> Self {
        let x0 = x_pin.read();
        let y0 = y_pin.read();

        Self {
            x_pin,
            y_pin,
            switch_pin,
            initial_pos: (x0, y0),
        }
    }

    pub fn get_pos(&self) -> (i16, i16) {
        let x = self.x_pin.read() as i16;
        let y = self.y_pin.read() as i16;

        let x0 = self.initial_pos.0 as i16;
        let y0 = self.initial_pos.1 as i16;

        // y0 - y is intentional to flip y-axis
        (x - x0, y0 - y)
    }

    pub fn is_pressed(&self) -> bool {
        match self.switch_pin.read() {
            super::arduino::io::State::HIGH => false,
            super::arduino::io::State::LOW => true,
        }
    }
}
