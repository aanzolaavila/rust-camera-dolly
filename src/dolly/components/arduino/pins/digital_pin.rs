use arduino_hal::port::Pin;
use avr_hal_generic::port::{
    mode::{Input, PullUp},
    PinOps,
};

use crate::dolly::components::arduino::io::DigitalRead;

pub struct DigitalInputPin<PIN: PinOps> {
    pin: Pin<Input<PullUp>, PIN>,
}

impl<PIN: PinOps> DigitalInputPin<PIN> {
    pub fn new(pin: Pin<Input<PullUp>, PIN>) -> impl DigitalRead {
        Self { pin }
    }
}

impl<T: PinOps> DigitalRead for DigitalInputPin<T> {
    fn read(&self) -> bool {
        self.pin.is_high()
    }
}
