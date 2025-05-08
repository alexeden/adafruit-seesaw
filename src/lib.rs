#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(const_evaluatable_unchecked, incomplete_features, rustdoc::bare_urls)]
#![feature(array_try_map, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
// Re-export rgb
pub use rgb;

pub mod bus;
pub mod devices;
pub mod modules;
pub mod prelude {
    pub use super::{
        devices::{SeesawDevice, SeesawDeviceInit},
        driver::DriverExt,
        modules::{
            adc::*, encoder::*, gpio::*, keypad::*, neopixel::*, status::*, timer::*, HardwareId,
        },
    };
}
mod driver;
use bus::{Bus, BusMutex, RefCellBus};
pub use driver::*;
use embedded_hal::{
    delay::DelayNs,
    i2c,
    i2c::{ErrorType, I2c},
};
use modules::HardwareId;

pub type SeesawRefCell<BUS> = Seesaw<RefCellBus<BUS>>;

#[cfg(feature = "std")]
pub type SeesawStdMutex<BUS> = Seesaw<std::sync::Mutex<BUS>>;

/// The owner of the driver from which new seesaw devices can be created.
/// Use this with a custom bus implementation from this crate.
///
/// If you instead wish to use [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus),
/// then you should use [`DirectI2cSeesaw`].
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

/// Unlike [`Seesaw`] this does *not* implement a custom bus.
/// Instead, you either use a single device with it or you use
/// [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus).
pub struct DirectI2cSeesaw<DELAY, I2C> {
    delay: DELAY,
    i2c: I2C,
}

impl<DELAY, I2C> DirectI2cSeesaw<DELAY, I2C> {
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Self { delay, i2c }
    }
}

// Delay implementation
impl<DELAY, I2C> DelayNs for DirectI2cSeesaw<DELAY, I2C>
where
    DELAY: DelayNs,
    I2C: I2c,
{
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns)
    }
}

impl<DELAY, I2C> ErrorType for DirectI2cSeesaw<DELAY, I2C>
where
    DELAY: DelayNs,
    I2C: I2c,
{
    type Error = I2C::Error;
}

impl<DELAY, I2C> I2c for DirectI2cSeesaw<DELAY, I2C>
where
    DELAY: DelayNs,
    I2C: I2c,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.i2c.transaction(address, operations)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}
