use arduino_hal::port::mode::{Analog, Input, PullUp};
use arduino_hal::port::Pin;

struct Joystick<PIN1, PIN2, PIN3> {
    x_pin: Pin<Analog, PIN1>,
    y_pin: Pin<Analog, PIN2>,
    switch_pin: Pin<Input<PullUp>, PIN3>,
}

impl<PIN1, PIN2, PIN3> Joystick<PIN1, PIN2, PIN3> {
    pub fn new(
        x_pin: Pin<Analog, PIN1>,
        y_pin: Pin<Analog, PIN2>,
        switch_pin: Pin<Input<PullUp>, PIN3>,
    ) -> Self {
        Joystick {
            x_pin,
            y_pin,
            switch_pin,
        }
    }
}
