#![no_std]
#![no_main]

use arduino_hal::hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;

use crate::dolly::components::arduino::adc_manager::AdcManager;
use crate::dolly::components::arduino::pins::analog_pin::AnalogInputPin;
use crate::dolly::components::arduino::pins::digital_pin::DigitalInputPin;
use crate::dolly::components::joystick::JoystickHAL;
use crate::dolly::Joystick;

mod dolly;
mod serial;

#[cfg(not(doc))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .unwrap_infallible();
    }

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(30);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let console = arduino_hal::default_serial!(dp, pins, 57600);
    serial::put_console(console);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    println!("Camera Dolly started ...");

    let in_led = pins.d9.into_output();
    let out_led = pins.d8.into_output();
    let mut builtin_led = pins.d13.into_output();

    let mut dir_pins = [
        in_led.downgrade(),
        pins.d2.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
    ];

    let mut step_pins = [
        out_led.downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
    ];

    let joy_switch_pin = pins.d10.into_pull_up_input();
    let joy_x = pins.a0.into_analog_input(&mut adc);
    let joy_y = pins.a1.into_analog_input(&mut adc);

    AdcManager::initialize(adc);

    let analog_x_pos = AnalogInputPin::new(joy_x.into_channel(), AdcManager::new());
    let analog_y_pos = AnalogInputPin::new(joy_y.into_channel(), AdcManager::new());
    let pull_up_switch_pin = DigitalInputPin::new(joy_switch_pin);

    let joystick = JoystickHAL::new(analog_x_pos, analog_y_pos, pull_up_switch_pin);

    while joystick.is_pressed() {
        let pos = joystick.get_pos();
        println!("X: {} Y: {}", pos.0, pos.1);
        arduino_hal::delay_ms(200);
    }

    println!("Joy Switch pressed ...");

    const CYCLE_PULSES: u32 = 200;
    const PULSE_DELAY: u32 = 500;

    loop {
        set_pins_state(&mut dir_pins, PinState::HIGH);
        builtin_led.set_high();

        for _ in 0..CYCLE_PULSES {
            set_pins_state(&mut step_pins, PinState::HIGH);
            arduino_hal::delay_us(PULSE_DELAY);
            set_pins_state(&mut step_pins, PinState::LOW);
            arduino_hal::delay_us(PULSE_DELAY);
        }

        arduino_hal::delay_ms(1000);

        set_pins_state(&mut dir_pins, PinState::LOW);
        builtin_led.set_low();
        for _ in 0..2 * CYCLE_PULSES {
            set_pins_state(&mut step_pins, PinState::HIGH);
            arduino_hal::delay_us(PULSE_DELAY);
            set_pins_state(&mut step_pins, PinState::LOW);
            arduino_hal::delay_us(PULSE_DELAY);
        }

        arduino_hal::delay_ms(1000);
    }
}

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
