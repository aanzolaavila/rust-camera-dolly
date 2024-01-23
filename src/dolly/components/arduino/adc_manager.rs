use core::cell::RefCell;

use avr_device::interrupt::Mutex;

use super::{AdcConcreteType, ChannelType};

static mut GLOBAL_ADC: Mutex<RefCell<Option<AdcConcreteType>>> = Mutex::new(RefCell::new(None));

pub struct AdcManager;

impl AdcManager {
    pub fn initialize(adc: AdcConcreteType) {
        // this could be handled better to check double initiation
        unsafe {
            GLOBAL_ADC = Mutex::new(RefCell::new(Some(adc)));
        }
    }

    pub fn new() -> Self {
        avr_device::interrupt::free(|cs| {
            let adc = unsafe { GLOBAL_ADC.borrow(cs).borrow() };
            if adc.is_none() {
                panic!("Adc Manager is not initialized");
            }
        });
        Self {}
    }

    pub fn analog_read(&self, chan: &ChannelType) -> u16 {
        let mut value: u16 = 0;
        avr_device::interrupt::free(|cs| unsafe {
            let mut adcopt = GLOBAL_ADC.borrow(cs).borrow_mut();
            let adc = adcopt.as_mut();
            value = adc.unwrap().read_blocking(chan);
        });
        value
    }
}
