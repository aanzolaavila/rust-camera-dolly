use arduino_hal::hal::{
    port::{PB2, PB3, PC0, PC1},
    Atmega,
};
use avr_hal_generic::{
    adc::{Adc, Channel},
    clock::MHz16,
};

pub mod adc_manager;
pub mod io;
pub mod pins;

pub type HType = Atmega;
pub type AdcType = arduino_hal::pac::ADC;
pub type Clock = MHz16;
pub type AdcConcreteType = Adc<HType, AdcType, Clock>;
pub type ChannelType = Channel<HType, AdcType>;

pub type IRPin = PB3;
pub type JoySwitchPin = PB2;
pub type JoyXPin = PC0;
pub type JoyYPin = PC1;
