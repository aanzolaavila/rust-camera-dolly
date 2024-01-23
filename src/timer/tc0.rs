/*
* REFERENCE
* - https://blog.rahix.de/005-avr-hal-millis/
* - https://github.com/jkristell/infrared/blob/master/examples/arduino_uno/src/bin/external-interrupt.rs
*/

use core::cell::Cell;

use avr_device::interrupt::Mutex;

static COUNTER: Mutex<Cell<u64>> = Mutex::new(Cell::new(0));
static CLOCK_TC0: ClockTC0 = ClockTC0::new();

// TODO: Test with TIMER0, TIMER1, TIMER0 and TIMER1 for implementing this Clock

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    CLOCK_TC0.tick();
}

pub struct ClockTC0;

impl ClockTC0 {
    pub const CPU_FREQ: u32 = 16_000_000; // 16 MHz
    pub const TARGET_FREQ: u32 = 20_000; // 20 KHz
    pub const PRESCALER: u32 = 8;
    pub const TIMER_COUNTS: u32 = (Self::CPU_FREQ / Self::TARGET_FREQ / Self::PRESCALER) - 1;

    pub const fn new() -> Self {
        Self {}
    }

    pub fn start(&self, tc0: arduino_hal::pac::TC0) {
        // Configure the timer for the above interval (in CTC mode)
        tc0.tccr0a.write(|w| w.wgm0().ctc());
        tc0.ocr0a.write(|w| w.bits(Self::TIMER_COUNTS as u8));
        tc0.tccr0b.write(|w| match Self::PRESCALER {
            8 => w.cs0().prescale_8(),
            64 => w.cs0().prescale_64(),
            256 => w.cs0().prescale_256(),
            1024 => w.cs0().prescale_1024(),
            _ => panic!(),
        });

        // Enable interrupt
        tc0.timsk0.write(|w| w.ocie0a().set_bit());
    }

    pub fn now(&self) -> u32 {
        avr_device::interrupt::free(|cs| COUNTER.borrow(cs).get()) as u32
    }

    pub fn tick(&self) {
        avr_device::interrupt::free(|cs| {
            let c = COUNTER.borrow(cs);
            let v = c.get();
            c.set(v.wrapping_add(1));
        });
    }
}
