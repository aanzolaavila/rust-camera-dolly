/*
* REFERENCE
* - https://blog.rahix.de/005-avr-hal-millis/
* - https://github.com/jkristell/infrared/blob/master/examples/arduino_uno/src/bin/external-interrupt.rs
*/

use core::cell::Cell;

use avr_device::interrupt::Mutex;

use crate::println;

static COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static CLOCK_TC1: ClockTC1 = ClockTC1::new();

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    CLOCK_TC1.tick();
}

pub struct ClockTC1;

impl ClockTC1 {
    pub const CPU_FREQ: u32 = 16_000_000; // 16 MHz
    pub const TARGET_FREQ: u32 = 2000;
    pub const PRESCALER: u32 = 8;
    pub const TIMER_COUNTS: u32 = (Self::CPU_FREQ / Self::TARGET_FREQ / Self::PRESCALER) - 1;
    pub const CORRECTION: u32 = 2413;
    pub const INCREMENT: u32 = 1_000_000 / Self::TARGET_FREQ;

    pub const fn new() -> Self {
        Self {}
    }

    pub fn start(&self, tc1: arduino_hal::pac::TC1) {
        if Self::TIMER_COUNTS > u16::max_value() as u32 {
            panic!();
        }

        let tccr1a = tc1.tccr1a.read().wgm1().bits();
        let tccr1b = tc1.tccr1b.read().bits();
        let ocr1a = tc1.ocr1a.read().bits();
        let timsk1 = tc1.timsk1.read().bits();

        println!(
            "tccr1a: {} | tccr1b: {} | ocr1a: {} | timsk1: {}",
            tccr1a, tccr1b, ocr1a, timsk1
        );

        tc1.tccr1a.reset();
        tc1.tccr1b.reset();
        tc1.tcnt1.reset();

        const CTC: u8 = 0b10;
        tc1.tccr1a.write(|w| w.wgm1().bits(CTC));
        tc1.tccr1b.write(|w| match Self::PRESCALER {
            1 => w.cs1().direct(),
            8 => w.cs1().prescale_8(),
            64 => w.cs1().prescale_64(),
            256 => w.cs1().prescale_256(),
            1024 => w.cs1().prescale_1024(),
            _ => panic!(),
        });
        tc1.ocr1a.write(|w| w.bits(Self::TIMER_COUNTS as u16));
        tc1.timsk1.write(|w| w.ocie1a().set_bit());

        let tccr1a = tc1.tccr1a.read().wgm1().bits();
        let tccr1b = tc1.tccr1b.read().bits();
        let ocr1a = tc1.ocr1a.read().bits();
        let timsk1 = tc1.timsk1.read().bits();

        println!(
            "tccr1a: {} | tccr1b: {} | ocr1a: {} | timsk1: {}",
            tccr1a, tccr1b, ocr1a, timsk1
        );
    }

    pub fn now(&self) -> u32 {
        avr_device::interrupt::free(|cs| COUNTER.borrow(cs).get()) / Self::CORRECTION as u32
    }

    pub fn tick(&self) {
        avr_device::interrupt::free(|cs| {
            let c = COUNTER.borrow(cs);
            let v = c.get();
            c.set(v.wrapping_add(Self::INCREMENT));
        });
    }
}
