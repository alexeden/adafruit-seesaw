#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    const_evaluatable_unchecked,
    incomplete_features,
    rustdoc::bare_urls,
    deprecated
)]
#![cfg_attr(
    feature = "module_neopixel",
    feature(array_try_map, generic_const_exprs)
)]
// TODO improve the organization of the exports/visibility
// Re-export rgb
pub use rgb;

pub mod bus;
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
use bus::{Bus, BusMutex, RefCellBus};
pub use driver::*;
use embedded_hal::{delay::DelayNs, i2c::I2c};

#[deprecated(
    since = "0.11.1",
    note = "Use SeesawDriver instead. For bus sharing, use third-party crates, e.g. the \
            RefCellDevice struct from the embedded-hal-bus crate."
)]
pub type SeesawRefCell<BUS> = Seesaw<RefCellBus<BUS>>;

#[cfg(feature = "std")]
#[deprecated(
    since = "0.11.1",
    note = "Use SeesawDriver instead. For bus sharing, use third-party crates, e.g. the \
            MutexDevice struct from the embedded-hal-bus crate."
)]
pub type SeesawStdMutex<BUS> = Seesaw<std::sync::Mutex<BUS>>;

/// The owner of the driver from which new seesaw devices can be created
#[deprecated(
    since = "0.11.1",
    note = "Use SeesawDriver instead. The BusMutex trait and its implementing structs are to be \
            removed in favor of using third-party crates (e.g. embedded-hal-bus) for bus sharing. \
            For more information, see the documentation for the BusMutex trait."
)]
pub struct Seesaw<M>(M);

impl<DELAY, I2C, M> Seesaw<M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Seesaw(M::create((delay, i2c)))
    }

    pub fn acquire_driver(&self) -> Bus<'_, M> {
        Bus(&self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}
