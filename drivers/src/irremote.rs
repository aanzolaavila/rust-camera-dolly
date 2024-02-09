use core::cell::Cell;

use arduino_hal::port::Pin;
use avr_device::interrupt::Mutex;
use avr_hal_generic::port::mode::{Floating, Input};
use infrared::{
    cmd::AddressCommand,
    protocol::{nec::NecCommand, Nec},
    PeriodicPoll, Receiver,
};
use ufmt::{uDisplay, uwrite};

use super::arduino::IRPin;

type IrPin = Pin<Input<Floating>, IRPin>;
type IrProto = Nec;
type IrCmd = NecCommand;
static mut RECEIVER: Option<PeriodicPoll<IrProto, IrPin>> = None;
static CMD: Mutex<Cell<Option<IrCmd>>> = Mutex::new(Cell::new(None));

pub struct IRRemote {}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl uDisplay for Dir {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: avr_hal_generic::prelude::_ufmt_uWrite + ?Sized,
    {
        match self {
            Dir::Up => uwrite!(f, "Up"),
            Dir::Down => uwrite!(f, "Down"),
            Dir::Left => uwrite!(f, "Left"),
            Dir::Right => uwrite!(f, "Right"),
        }
    }
}

pub enum Command {
    Number(u8),
    Ok,
    Direction(Dir),
    Asterisc,
    Numeral,
}

impl uDisplay for Command {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: avr_hal_generic::prelude::_ufmt_uWrite + ?Sized,
    {
        match self {
            Command::Number(n) => uwrite!(f, "Number({})", n),
            Command::Ok => uwrite!(f, "Ok"),
            Command::Direction(dir) => uwrite!(f, "Direction({})", dir),
            Command::Asterisc => uwrite!(f, "Asterisc"),
            Command::Numeral => uwrite!(f, "Numeral"),
        }
    }
}

impl IRRemote {
    pub fn initialize(pin: IrPin) {
        const POLL_FREQ: u32 = 20_000;
        let ir = PeriodicPoll::with_pin(POLL_FREQ, pin);
        unsafe { RECEIVER.replace(ir) };
    }

    pub fn new() -> Self {
        if unsafe { RECEIVER.as_ref().is_none() } {
            panic!("IRRemote was not initialized");
        }

        Self {}
    }

    pub fn get_cmd(&self) -> Option<Command> {
        if let Some(cmd) = avr_device::interrupt::free(|cs| CMD.borrow(cs).take()) {
            let ans = match cmd.cmd {
                82 => Some(Command::Number(0)),
                22 => Some(Command::Number(1)),
                25 => Some(Command::Number(2)),
                13 => Some(Command::Number(3)),
                12 => Some(Command::Number(4)),
                24 => Some(Command::Number(5)),
                94 => Some(Command::Number(6)),
                08 => Some(Command::Number(7)),
                28 => Some(Command::Number(8)),
                90 => Some(Command::Number(9)),

                64 => Some(Command::Ok),

                68 => Some(Command::Direction(Dir::Left)),
                70 => Some(Command::Direction(Dir::Up)),
                67 => Some(Command::Direction(Dir::Right)),
                21 => Some(Command::Direction(Dir::Down)),

                66 => Some(Command::Asterisc),
                74 => Some(Command::Numeral),
                _ => None,
            };

            // lets clear it
            // avr_device::interrupt::free(|cs| {
            //     CMD.borrow(cs).set(None);
            // });

            return ans;
        }

        None
    }
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
