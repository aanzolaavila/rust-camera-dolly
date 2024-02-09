pub enum State {
    HIGH,
    LOW,
}

pub trait DigitalRead {
    fn read(&self) -> State;
}

pub trait AnalogRead {
    fn read(&self) -> u16;
}

pub trait DigitalWrite {
    fn write(&mut self, value: State);
    fn toggle(&mut self);
}

pub trait AnalogWrite {
    fn write(&mut self, value: u16);
}
