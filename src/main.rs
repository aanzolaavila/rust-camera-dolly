#![no_std]
#![no_main]

use arduino_hal::hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;

mod dolly;

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

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

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
