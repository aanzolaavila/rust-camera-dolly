/*
* REFERENCE
* - https://blog.rahix.de/005-avr-hal-millis/
* - https://github.com/jkristell/infrared/blob/master/examples/arduino_uno/src/bin/external-interrupt.rs
*/

use core::cell::Cell;

use avr_device::interrupt::Mutex;

static COUNTER_MICRO: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
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
    pub const TIME_PER_OVERFLOW: u32 =
        Self::PRESCALER * Self::TIMER_COUNTS as u32 / (Self::CPU_FREQ / 1000);

    pub const fn new() -> Self {
        Self {}
    }

    pub fn start(&self, tc0: arduino_hal::pac::TC0) {
        if Self::TIMER_COUNTS > u8::max_value() as u32 {
            panic!();
        }

        tc0.tccr0a.reset();
        tc0.tccr0a.reset();
        tc0.tcnt0.reset();

        // Configure the timer for the above interval (in CTC mode)
        tc0.tccr0a.write(|w| w.wgm0().ctc());
        tc0.tccr0b.write(|w| match Self::PRESCALER {
            1 => w.cs0().direct(),
            8 => w.cs0().prescale_8(),
            64 => w.cs0().prescale_64(),
            256 => w.cs0().prescale_256(),
            1024 => w.cs0().prescale_1024(),
            _ => panic!(),
        });
        tc0.ocr0a.write(|w| w.bits(Self::TIMER_COUNTS as u8));

        // Enable interrupt
        tc0.timsk0.write(|w| w.ocie0a().set_bit());
    }

    pub fn now(&self) -> u32 {
        avr_device::interrupt::free(|cs| COUNTER_MICRO.borrow(cs).get())
    }

    pub fn tick(&self) {
        avr_device::interrupt::free(|cs| {
            let c = COUNTER_MICRO.borrow(cs);
            let v = c.get();
            c.set(v.wrapping_add(1));
        });
    }
}
