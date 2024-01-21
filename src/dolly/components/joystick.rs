use crate::dolly::Joystick;

use super::arduino::io::{AnalogRead, DigitalRead};

pub struct JoystickHAL<APIN, PPIN>
where
    APIN: AnalogRead,
    PPIN: DigitalRead,
{
    x_pin: APIN,
    y_pin: APIN,
    switch_pin: PPIN,
    initial_pos: (u16, u16),
}

impl<APIN, PPIN> JoystickHAL<APIN, PPIN>
where
    APIN: AnalogRead,
    PPIN: DigitalRead,
{
    pub fn new(x_pin: APIN, y_pin: APIN, switch_pin: PPIN) -> impl Joystick {
        let x0 = x_pin.read();
        let y0 = y_pin.read();

        Self {
            x_pin,
            y_pin,
            switch_pin,
            initial_pos: (x0, y0),
        }
    }
}

impl<APIN, PPIN> Joystick for JoystickHAL<APIN, PPIN>
where
    APIN: AnalogRead,
    PPIN: DigitalRead,
{
    fn get_pos(&self) -> (i16, i16) {
        let x = self.x_pin.read() as i16;
        let y = self.y_pin.read() as i16;

        let x0 = self.initial_pos.0 as i16;
        let y0 = self.initial_pos.1 as i16;

        (x - x0, y - y0)
    }

    fn is_pressed(&self) -> bool {
        self.switch_pin.read()
    }
}
