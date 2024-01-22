use arduino_hal::port::Pin;
use avr_hal_generic::port::mode::{Input, Output};

use crate::dolly::components::arduino::io::{DigitalRead, DigitalWrite, State};

pub struct DigitalInput {
    pin: Pin<Input>,
}

impl DigitalInput {
    pub fn new(pin: Pin<Input>) -> Self {
        Self { pin }
    }
}

impl DigitalRead for DigitalInput {
    fn read(&self) -> State {
        if self.pin.is_high() {
            State::HIGH
        } else {
            State::LOW
        }
    }
}

pub struct DigitalOutput {
    pin: Pin<Output>,
    current_state: State,
}

impl DigitalOutput {
    pub fn new(pin: Pin<Output>) -> Self {
        Self {
            pin,
            current_state: State::LOW,
        }
    }
}

impl DigitalWrite for DigitalOutput {
    fn write(&mut self, value: State) {
        match value {
            State::HIGH => self.pin.set_high(),
            State::LOW => self.pin.set_low(),
        }
        self.current_state = value;
    }

    fn toggle(&mut self) {
        match self.current_state {
            State::HIGH => self.write(State::LOW),
            State::LOW => self.write(State::HIGH),
        }
    }
}
