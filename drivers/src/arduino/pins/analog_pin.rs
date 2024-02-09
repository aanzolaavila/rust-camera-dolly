use crate::arduino::{adc_manager::AdcManager, io::AnalogRead, ChannelType};

pub struct AnalogInput {
    chan: ChannelType,
    adc: AdcManager,
}

impl AnalogInput {
    pub fn new(chan: ChannelType, adc: AdcManager) -> Self {
        Self { chan, adc }
    }
}

impl AnalogRead for AnalogInput {
    fn read(&self) -> u16 {
        self.adc.analog_read(&self.chan)
    }
}
