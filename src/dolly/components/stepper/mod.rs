use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

#[allow(dead_code)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub trait Motor {
    fn set_direction(dir: Direction);
    fn run();
    fn stop();
}

pub struct Stepper {
    direction: Direction,
}

// TEMP

enum PinState {
    HIGH,
    LOW,
}

fn set_pins_state(pins: &mut [Pin<Output>], state: PinState) {
    for p in pins {
        match state {
            PinState::HIGH => {
                p.set_high();
            }
            PinState::LOW => {
                p.set_low();
            }
        }
    }
}

// const CYCLE_PULSES: u32 = 200;
// const PULSE_DELAY: u32 = 500;
//
// loop {
//     set_pins_state(&mut dir_pins, PinState::HIGH);
//     builtin_led.set_high();
//
//     for _ in 0..CYCLE_PULSES {
//         set_pins_state(&mut step_pins, PinState::HIGH);
//         arduino_hal::delay_us(PULSE_DELAY);
//         set_pins_state(&mut step_pins, PinState::LOW);
//         arduino_hal::delay_us(PULSE_DELAY);
//     }
//
//     arduino_hal::delay_ms(1000);
//
//     set_pins_state(&mut dir_pins, PinState::LOW);
//     builtin_led.set_low();
//     for _ in 0..2 * CYCLE_PULSES {
//         set_pins_state(&mut step_pins, PinState::HIGH);
//         arduino_hal::delay_us(PULSE_DELAY);
//         set_pins_state(&mut step_pins, PinState::LOW);
//         arduino_hal::delay_us(PULSE_DELAY);
//     }
//
//     arduino_hal::delay_ms(1000);
// }
