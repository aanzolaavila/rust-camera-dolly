# Camera Slider in Rust

**WIP** This is a personal project aimed to develop a Camera Slider with an Arduino
board.

The intention was to write as much as possible with Rust for embedded devices.
As it turns out, most of the development done for Rust in Arduino has been done
by the software community with no official support, this means that there are
missing parts that either have to be developed manually, or use bindgen to
include Arduino C API into the executable (with aid of `bindgen`).

## Circuit References

### Baseline
  - https://howtomechatronics.com/tutorials/arduino/how-to-control-stepper-motor-with-a4988-driver-and-arduino/
  - https://howtomechatronics.com/tutorials/arduino/diy-motorized-camera-slider-pan-tilt-head-project/

### A4988 Stepper Driver
 - https://www.hobby-hour.com/electronics/smdcalc.php
 - https://ardufocus.com/howto/a4988-motor-current-tuning/
 - https://www.youtube.com/watch?v=OpaUwWouyE0

### Arduino
  - **TODO**

## Software References

There was a lot of help from _ChatGPT_, it allowed me to have more insight
on problems that I have faced.

### Embedded Rust
  - https://docs.rust-embedded.org/book/intro/index.html
  - Arduino HAL (Hardware Abstraction Layer) https://github.com/Rahix/avr-hal

### Bindgen
  - **TODO**

### Millis function
  - https://blog.rahix.de/005-avr-hal-millis/
  - https://github.com/jkristell/infrared/blob/master/examples/arduino_uno/src/bin/external-interrupt.rs

## Build process

To have a consistent environment, and to avoid having package conflicts with
anything, I decided to have the entire build pipeline inside a Docker container.
I only wish I had decided to do this step at the very beginning, it would
have saved a lot of hours.

### Steps
  - **TODO**
