use core::time::Duration;

use crate::arduino::{
    io::{DigitalWrite, State},
    pins::digital_pin::DigitalOutput,
};

use crate::timer::Clock;
use ufmt::derive::uDebug;
use void::Void;

pub use accel_stepper::{Device, Driver, MultiDriver, SystemClock};

#[derive(Debug, Default, uDebug)]
pub struct TickingClock<C: Clock> {
    clock: C,
}

impl<C: Clock> SystemClock for TickingClock<C> {
    fn elapsed(&self) -> core::time::Duration {
        let millis = self.clock.now();
        Duration::from_millis(millis as u64)
    }
}

impl<C: Clock> TickingClock<C> {
    pub fn new(clock: C) -> Self {
        Self { clock }
    }
}

pub struct AccelDevice {
    step: DigitalOutput,
    direction: DigitalOutput,
    step_duration: Duration,
    previous_position: i64,
}

impl AccelDevice {
    pub fn new(step: DigitalOutput, direction: DigitalOutput, step_duration: Duration) -> Self {
        Self {
            step,
            direction,
            step_duration,
            previous_position: 0,
        }
    }

    fn forward(&mut self) {
        let step = &mut self.step;
        let dir = &mut self.direction;

        let dt = self.step_duration.as_micros();

        dir.write(State::LOW);
        step.write(State::HIGH);
        arduino_hal::delay_us(dt as u32);
        step.write(State::LOW);

        // serial_console::println!("forward pos: {}", self.previous_position);

        self.previous_position += 1;
    }

    fn backward(&mut self) {
        let step = &mut self.step;
        let dir = &mut self.direction;

        let dt = self.step_duration.as_micros();

        dir.write(State::HIGH);
        step.write(State::HIGH);
        arduino_hal::delay_us(dt as u32);
        step.write(State::LOW);
        dir.write(State::LOW);

        // serial_console::println!("backward pos: {}", self.previous_position);

        self.previous_position -= 1;
    }
}

impl Device for AccelDevice {
    type Error = Void;

    fn step(&mut self, ctx: &accel_stepper::StepContext) -> Result<(), Self::Error> {
        let diff = ctx.position - self.previous_position;

        if diff > 0 {
            self.forward();
        } else if diff < 0 {
            self.backward();
        }

        Ok(())
    }
}
