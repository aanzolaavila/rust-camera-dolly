#[allow(dead_code)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub trait Motor {
    fn set_direction(dir: Direction);
    fn run();
    fn stop();
}

pub struct Stepper {
    direction: Direction,
}
