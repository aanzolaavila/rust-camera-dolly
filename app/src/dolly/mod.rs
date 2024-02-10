use core::fmt::Write;
use core::ops::Range;

use arduino_core::println;

use binding::liquid_crystal::{Backlight, LiquidCrystal};
use drivers::{
    arduino::{
        io::{DigitalWrite, State},
        pins::digital_pin::DigitalOutput,
    },
    joystick::Joystick,
    timer::{tc0::ClockTC0, tc1::ClockTC1},
};

pub struct Settings {
    pub tc0_clock: ClockTC0,
    pub tc1_clock: ClockTC1,
    pub joystick: Joystick,
    pub builtin_led: DigitalOutput,
    pub in_led: DigitalOutput,
    pub out_led: DigitalOutput,
    pub lcd: LiquidCrystal,
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
}

impl Dolly {
    pub fn new(cfg: Settings) -> Self {
        Self { cfg }
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

        let mut t = (self.cfg.tc0_clock.now() - 5_000) / 1000 as u32;
        println!("Current time: {}", t);

        let mut lcd = &mut self.cfg.lcd;

        lcd.backlight(Backlight::On);
        lcd.clear();

        let h = t / 3600;
        t %= 3600;
        let m = t / 60;
        t %= 60;
        let s = t;

        let _ = lcd.set_cursor(0, 0);
        let _ = write!(lcd, "Current time:");
        let _ = lcd.set_cursor(0, 1);
        let _ = write!(lcd, "{h}h {m}m {s}s");

        arduino_hal::delay_ms(1000);
    }
}
