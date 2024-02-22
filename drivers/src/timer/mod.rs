pub mod tc0;
pub mod tc1;

pub trait Clock {
    fn now(&self) -> u32;
}
