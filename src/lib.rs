#![doc = include_str!("../README.md")]
#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features, rustdoc::bare_urls)]
#![cfg_attr(feature = "module_neopixel", feature(generic_const_exprs))]

use core::fmt::{Display, Formatter};
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}

impl<E> Display for SeesawError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            SeesawError::I2c(_) => f.write_str("I2C error"),
            SeesawError::InvalidHardwareId(id) => write!(f, "invalid hardware id: {id}"),
        }
    }
}

impl<E: core::fmt::Debug + core::error::Error + 'static> core::error::Error for SeesawError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            SeesawError::I2c(err) => Some(err),
            _ => None,
        }
    }
}
