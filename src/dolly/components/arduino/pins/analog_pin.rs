use crate::dolly::components::arduino::{adc_manager::AdcManager, io::AnalogRead, ChannelType};

pub struct AnalogInputPin {
    chan: ChannelType,
    adc: AdcManager,
}

impl AnalogInputPin {
    pub fn new(chan: ChannelType, adc: AdcManager) -> impl AnalogRead {
        Self { chan, adc }
    }
}

impl AnalogRead for AnalogInputPin {
    fn read(&self) -> u16 {
        self.adc.analog_read(&self.chan)
    }
}
