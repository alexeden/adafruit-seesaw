#![doc = include_str!("../README.md")]
#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features, rustdoc::bare_urls)]
#![cfg_attr(feature = "module_neopixel", feature(generic_const_exprs))]

// Re-export rgb
pub use rgb;

pub mod devices;
pub mod modules;
pub mod prelude {
    #[cfg(feature = "module_adc")]
    pub use super::modules::adc::*;
    #[cfg(feature = "module_encoder")]
    pub use super::modules::encoder::*;
    #[cfg(feature = "module_gpio")]
    pub use super::modules::gpio::*;
    #[cfg(feature = "module_keypad")]
    pub use super::modules::keypad::*;
    #[cfg(feature = "module_neopixel")]
    pub use super::modules::neopixel::*;
    #[cfg(feature = "module_timer")]
    pub use super::modules::timer::*;
    pub use super::{
        devices::{SeesawDevice, SeesawDeviceInit},
        driver::{DriverExt, SeesawDriver},
        modules::{status::*, HardwareId},
    };
}
mod driver;
pub use driver::*;

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}
