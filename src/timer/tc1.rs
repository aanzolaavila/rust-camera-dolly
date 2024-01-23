/*
* REFERENCE
* - https://blog.rahix.de/005-avr-hal-millis/
* - https://github.com/jkristell/infrared/blob/master/examples/arduino_uno/src/bin/external-interrupt.rs
*/

use core::{
    cell::Cell,
    sync::atomic::{AtomicU16, Ordering},
};

use avr_device::interrupt::Mutex;

static ATOMIC_COUNTER: AtomicU16 = AtomicU16::new(0);
static COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static CLOCK_TC1: ClockTC1 = ClockTC1::new();

// TODO: Test with TIMER0, TIMER1, TIMER0 and TIMER1 for implementing this Clock

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    CLOCK_TC1.tick();
}

pub struct ClockTC1;

impl ClockTC1 {
    pub const CPU_FREQ: u32 = 16_000_000; // 16 MHz
    pub const TARGET_FREQ: u32 = 1_000; // 1 KHz
    pub const PRESCALER: u32 = 64;
    pub const TIMER_COUNTS: u32 = (Self::CPU_FREQ / Self::TARGET_FREQ / Self::PRESCALER) - 1;
    // pub const MILLIS_INCREMENT: u32 = 1_000 * Self::PRESCALER * Self::TIMER_COUNTS / Self::CPU_FREQ; //
    pub const MILLIS_INCREMENT: u32 = 2;

    pub const fn new() -> Self {
        Self {}
    }

    pub fn start(&self, tc1: arduino_hal::pac::TC1) {
        tc1.tccr1a.reset();
        tc1.tccr1b.reset();

        const CTC: u8 = 0b10;
        tc1.tccr1a.write(|w| w.wgm1().bits(CTC));
        tc1.tccr1b.write(|w| match Self::PRESCALER {
            8 => w.cs1().prescale_8(),
            64 => w.cs1().prescale_64(),
            256 => w.cs1().prescale_256(),
            1024 => w.cs1().prescale_1024(),
            _ => panic!(),
        });
        tc1.ocr1a.write(|w| w.bits(Self::TIMER_COUNTS as u16));
        tc1.timsk1.write(|w| w.ocie1a().set_bit());
    }

    pub fn now(&self) -> u32 {
        // avr_device::interrupt::free(|cs| COUNTER.borrow(cs).get()) as u32
        ATOMIC_COUNTER.load(Ordering::Relaxed) as u32
    }

    pub fn tick(&self) {
        // avr_device::interrupt::free(|cs| {
        //     let c = COUNTER.borrow(cs);
        //     let v = c.get();
        //     c.set(v.wrapping_add(1));
        // });
        ATOMIC_COUNTER.fetch_add(Self::MILLIS_INCREMENT as u16, Ordering::Relaxed);
    }
}
