use core::ops::Range;

use crate::{
    println,
    timer::{tc0::ClockTC0, tc1::ClockTC1},
};

use self::components::{
    arduino::{
        io::{DigitalWrite, State},
        pins::digital_pin::DigitalOutput,
    },
    irremote::IRRemote,
    joystick::Joystick,
};

pub mod components;

pub struct Settings {
    pub tc0_clock: ClockTC0,
    pub tc1_clock: ClockTC1,
    pub irremote: IRRemote,
    pub joystick: Joystick,
    pub builtin_led: DigitalOutput,
    pub in_led: DigitalOutput,
    pub out_led: DigitalOutput,
}

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

pub struct Dolly {
    cfg: Settings,
    prev_tc0: u32,
    prev_tc0_millis: u32,
    prev_tc1: u32,
}

impl Dolly {
    pub fn new(cfg: Settings) -> Self {
        Self {
            cfg,
            prev_tc0: 0,
            prev_tc0_millis: 0,
            prev_tc1: 0,
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
        self.cfg.builtin_led.toggle();

        let pos = self.cfg.joystick.get_pos();
        let x = Self::map(pos.0 as i32, (-500, 500), (-200, 200));
        let y = Self::map(pos.1 as i32, (-500, 500), (-200, 200));
        // println!("Joystick: ({}, {})", x, y);

        match x > 0 {
            true => self.cfg.in_led.write(State::HIGH),
            false => self.cfg.in_led.write(State::LOW),
        }

        match y > 0 {
            true => self.cfg.out_led.write(State::HIGH),
            false => self.cfg.out_led.write(State::LOW),
        }

        if let Some(cmd) = self.cfg.irremote.get_cmd() {
            println!("Cmd: {}", cmd);
        }

        let tc0_clock = self.cfg.tc0_clock.now();
        let tc0_clock_millis = self.cfg.tc0_clock.millis();
        let tc1_clock = self.cfg.tc1_clock.now();

        let tc0_clock_diff = tc0_clock - self.prev_tc0;
        let tc0_clock_millis_diff = tc0_clock_millis - self.prev_tc0_millis;
        let tc1_clock_diff = tc1_clock - self.prev_tc1;

        self.prev_tc0 = tc0_clock;
        self.prev_tc0_millis = tc0_clock_millis;
        self.prev_tc1 = tc1_clock;

        // println!("Clock TC0: {}", tc0_clock_diff);
        // println!("Clock TC0 millis: {}", tc0_clock_millis_diff);
        println!("Clock TC1: {}", tc1_clock_diff);
        arduino_hal::delay_ms(10_000);
    }
}
