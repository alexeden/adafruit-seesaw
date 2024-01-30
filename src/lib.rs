#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(const_evaluatable_unchecked, incomplete_features, rustdoc::bare_urls)]
#![feature(array_try_map, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
pub mod bus;
pub mod devices;
pub mod modules;
pub mod prelude {
    pub use super::{
        devices::{SeesawDevice, SeesawDeviceInit},
        driver::DriverExt,
        modules::{adc::*, encoder::*, gpio::*, neopixel::*, status::*, timer::*},
    };
}
mod driver;
use bus::{Bus, BusMutex, RefCellBus};
pub use driver::*;
use embedded_hal::{delay::DelayNs, i2c::I2c};
use modules::HardwareId;

pub type SeesawRefCell<BUS> = Seesaw<RefCellBus<BUS>>;

#[cfg(feature = "std")]
pub type SeesawStdMutex<BUS> = Seesaw<std::sync::Mutex<BUS>>;

/// The owner of the driver from which new seesaw devices can be created
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
