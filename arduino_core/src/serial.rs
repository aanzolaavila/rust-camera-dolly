use core::cell::RefCell;
use core::option::Option;
use core::option::Option::{None, Some};

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
type Container = avr_device::interrupt::Mutex<RefCell<Option<Console>>>;
pub static CONSOLE: Container = avr_device::interrupt::Mutex::new(RefCell::new(None));

#[allow(unused_macros)]
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = arduino_core::serial::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = arduino_core::serial::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

pub fn put_console(console: Console) {
    avr_device::interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
