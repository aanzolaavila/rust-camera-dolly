#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::Cell;

use arduino_hal::hal::port::PB3;
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;
use avr_device::atmega328p::tc0::tccr0b::CS0_A;
use avr_device::interrupt::Mutex;
use avr_hal_generic::port::mode::{Floating, Input};
use dolly::components::arduino::io::{DigitalWrite, State};
use infrared::protocol::nec::NecCommand;
use infrared::protocol::Nec;
use infrared::PeriodicPoll;

use crate::dolly::components::arduino::adc_manager::AdcManager;
use crate::dolly::components::arduino::pins::analog_pin::AnalogInput;
use crate::dolly::components::arduino::pins::digital_pin::{DigitalInput, DigitalOutput};
use crate::dolly::components::joystick::Joystick;

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

/*
* TODO LIST
* [x] Implement Switch
* [ ] Implement IR Remote
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

type IrPin = Pin<Input<Floating>, PB3>;
type IrProto = Nec;
type IrCmd = NecCommand;
static mut RECEIVER: Option<PeriodicPoll<IrProto, IrPin>> = None;
static CMD: Mutex<Cell<Option<IrCmd>>> = Mutex::new(Cell::new(None));

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    {
        let console = arduino_hal::default_serial!(dp, pins, 57600);
        serial::put_console(console);
    }
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // TOP = CPU_FREQ / TARGET_FREG / PRESCALER - 1
    // 16_000_000 / 20_000 / 8 - 1 = 99
    timer_start(dp.TC0, CS0_A::PRESCALE_8, 99);

    println!("Camera Dolly setup ...");

    let joy_switch_pin = pins.d10.into_pull_up_input();
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
        let pin = pins.d9.into_output();
        DigitalOutput::new(pin.downgrade())
    };
    let out_led = {
        let pin = pins.d8.into_output();
        DigitalOutput::new(pin.downgrade())
    };

    let mut _dolly = dolly::Dolly::new(builtin_led, joystick, in_led, out_led);

    const POLL_FREQ: u32 = 20_000;
    let ir = PeriodicPoll::with_pin(POLL_FREQ, pins.d11);

    unsafe { RECEIVER.replace(ir) };

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    println!("Started ...");
    loop {
        if let Some(cmd) = avr_device::interrupt::free(|cs| CMD.borrow(cs).take()) {
            println!(
                "Cmd: Address: {}, Command: {}, repeat: {}\r",
                cmd.addr, cmd.cmd, cmd.repeat
            );
        }
    }

    // loop {
    //     dolly.run();
    //     arduino_hal::delay_ms(32);
    // }
}

fn timer_start(tc0: arduino_hal::pac::TC0, prescaler: CS0_A, top: u8) {
    // Configure the timer for the above interval (in CTC mode)
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(top));
    tc0.tccr0b.write(|w| w.cs0().variant(prescaler));

    // Enable interrupt
    tc0.timsk0.write(|w| w.ocie0a().set_bit());
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    let recv = unsafe { RECEIVER.as_mut().unwrap() };

    if let Ok(Some(cmd)) = recv.poll() {
        // Command received

        avr_device::interrupt::free(|cs| {
            let cell = CMD.borrow(cs);
            cell.set(Some(cmd));
        });
    }
}
