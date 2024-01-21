pub trait DigitalRead {
    fn read(&self) -> bool;
}

pub trait AnalogRead {
    fn read(&self) -> u16;
}

pub trait DigitalWrite {
    fn write(&mut self, value: bool);
}

pub trait AnalogWrite {
    fn write(&mut self, value: u16);
}
