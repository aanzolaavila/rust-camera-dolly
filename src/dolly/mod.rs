pub mod components;
pub mod stepper;

pub trait Joystick {
    fn get_pos(&self) -> (i16, i16);
    fn is_pressed(&self) -> bool;
}
