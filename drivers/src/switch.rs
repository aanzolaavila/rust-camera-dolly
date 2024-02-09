use super::arduino::io::{DigitalRead, State};

pub struct Switch<PIN>
where
    PIN: DigitalRead,
{
    pin: PIN,
}

impl<PIN> Switch<PIN>
where
    PIN: DigitalRead,
{
    pub fn new(pin: PIN) -> Self {
        Self { pin }
    }

    pub fn is_pressed(&self) -> bool {
        match self.pin.read() {
            State::HIGH => false,
            State::LOW => true,
        }
    }
}
