use core::ops::Range;

use crate::println;

use self::components::{
    arduino::{
        io::{AnalogRead, DigitalRead, DigitalWrite, State},
        pins::digital_pin::DigitalOutput,
    },
    joystick::Joystick,
};

pub mod components;

struct Position {
    slider: i32,
    pan: i32,
    tilt: i32,
}

enum DollyState {
    SetInitPos,
    SetEndPos(Position),
    GotoInit {
        range: Range<Position>,
        current: Position,
    },
    Ready(Range<Position>),
    Moving {
        range: Range<Position>,
        current: Position,
    },
}

pub struct Dolly<AR, DR>
where
    AR: AnalogRead,
    DR: DigitalRead,
{
    joystick: Joystick<AR, DR>,
    builtin_led: DigitalOutput,
    in_led: DigitalOutput,
    out_led: DigitalOutput,
}

impl<AR, DR> Dolly<AR, DR>
where
    AR: AnalogRead,
    DR: DigitalRead,
{
    pub fn new(
        builtin_led: DigitalOutput,
        joystick: Joystick<AR, DR>,
        in_led: DigitalOutput,
        out_led: DigitalOutput,
    ) -> Self {
        Self {
            joystick,
            builtin_led,
            in_led,
            out_led,
        }
    }

    fn map(value: i32, from_range: (i32, i32), to_range: (i32, i32)) -> i32 {
        let (out_min, out_max) = to_range;
        let (in_min, in_max) = from_range;

        let mut v = libm::fmaxf(in_min as f32, libm::fminf(in_max as f32, value as f32));

        v -= in_min as f32;
        v *= (out_max - out_min) as f32;
        v /= (in_max - in_min) as f32;
        v += out_min as f32;

        v as i32
    }

    pub fn run(&mut self) {
        self.builtin_led.toggle();

        let pos = self.joystick.get_pos();
        let x = Self::map(pos.0 as i32, (-500, 500), (-200, 200));
        let y = Self::map(pos.1 as i32, (-500, 500), (-200, 200));
        println!("Joystick: ({}, {})", x, y);

        if x <= 0 {
            self.in_led.write(State::LOW);
        } else {
            self.in_led.write(State::HIGH);
        }

        if y <= 0 {
            self.out_led.write(State::LOW);
        } else {
            self.out_led.write(State::HIGH);
        }
    }
}
