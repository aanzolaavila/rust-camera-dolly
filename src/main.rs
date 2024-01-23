#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{prelude::*, Peripherals};
use dolly::components::arduino::io::{DigitalWrite, State};
use dolly::components::irremote::IRRemote;

use crate::dolly::components::arduino::adc_manager::AdcManager;
use crate::dolly::components::arduino::pins::analog_pin::AnalogInput;
use crate::dolly::components::arduino::pins::digital_pin::{DigitalInput, DigitalOutput};
use crate::dolly::components::joystick::Joystick;
use crate::timer::tc0::ClockTC0;
use crate::timer::tc1::ClockTC1;

mod dolly;
mod serial;
mod timer;

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

/*
* TODO LIST
* [x] Implement Switch
* [x] Implement IR Remote
* [ ] Implement Clock (millis function equivalent)
* [ ] Implement LCD Display
* [ ] Implement Stepper
* [?] Implement Potentiometer
* */

fn signal_hardware_is_ready(bled: &mut DigitalOutput) {
    for _ in 0..10 {
        bled.write(State::HIGH);
        arduino_hal::delay_ms(25);
        bled.write(State::LOW);
        arduino_hal::delay_ms(25);
    }

    bled.write(State::LOW);
}

// For IR Remote interrupts
// Reference: https://github.com/steveio/arduino/blob/master/PinChangeInterrupts/PinChangeInterrupts.ino
// https://www.arduino.cc/reference/en/language/functions/external-interrupts/attachinterrupt/
fn configure_interrupts(dp: &Peripherals) {
    const PCICR_PORTB: u8 = 0b001; // turn on port B (PCINT0 – PCINT7)
    const PCICR_PORTC: u8 = 0b010; // turn on port C (PCINT8 – PCINT14)
    const PCICR_PORTD: u8 = 0b100; // turn on port D (PCINT16 – PCINT23)

    avr_device::interrupt::free(|_| {
        // WARNING: Change this if IR Remote port changes
        // Port D2 (IR Remote is port PD2 / PCINT18)
        dp.EXINT.pcicr.write(|w| unsafe { w.bits(PCICR_PORTD) });
        dp.EXINT.pcmsk2.write(|w| w.bits(0b100));
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    avr_device::interrupt::disable();

    let dp = arduino_hal::Peripherals::take().unwrap();

    configure_interrupts(&dp);
    let tc0_clock = ClockTC0::new();
    tc0_clock.start(dp.TC0);

    let tc1_clock = ClockTC1::new();
    tc1_clock.start(dp.TC1);

    let pins = arduino_hal::pins!(dp);
    {
        let console = arduino_hal::default_serial!(dp, pins, 57600);
        serial::put_console(console);
    }
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    println!("Camera Dolly setup ...");

    let joy_switch_pin = pins.d4.into_pull_up_input();
    let joy_x = pins.a0.into_analog_input(&mut adc);
    let joy_y = pins.a1.into_analog_input(&mut adc);

    AdcManager::initialize(adc);

    let analog_x_pos = AnalogInput::new(joy_x.into_channel(), AdcManager::new());
    let analog_y_pos = AnalogInput::new(joy_y.into_channel(), AdcManager::new());
    let pull_up_switch_pin = DigitalInput::new(
        joy_switch_pin
            .into_pull_up_input()
            .downgrade()
            .forget_imode(),
    );

    let joystick = Joystick::new(analog_x_pos, analog_y_pos, pull_up_switch_pin);

    let mut builtin_led = {
        let pin = pins.d13.into_output();
        DigitalOutput::new(pin.downgrade())
    };
    signal_hardware_is_ready(&mut builtin_led);

    let in_led = {
        let pin = pins.d6.into_output();
        DigitalOutput::new(pin.downgrade())
    };
    let out_led = {
        let pin = pins.d5.into_output();
        DigitalOutput::new(pin.downgrade())
    };

    IRRemote::initialize(pins.d2);
    let irremote = IRRemote::new();

    let settings = dolly::Settings {
        tc0_clock,
        tc1_clock,
        irremote,
        joystick,
        builtin_led,
        in_led,
        out_led,
    };
    let mut dolly = dolly::Dolly::new(settings);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    println!("Started ...");

    loop {
        dolly.run();
    }
}
