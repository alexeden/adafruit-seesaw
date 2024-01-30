#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(array_try_map, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
pub mod bus;
pub mod devices;
pub mod modules;
pub mod mutex;
pub mod prelude {
    pub use super::{
        devices::*,
        driver::DriverExt,
        modules::{adc::*, encoder::*, gpio::*, neopixel::*, status::*, timer::*},
        SeesawDevice, SeesawDeviceInit,
    };
}
pub use devices::*;
pub use driver::*;

mod driver;
mod macros;

use bus::Bus;
use embedded_hal::{delay::DelayNs, i2c::I2c};
use modules::HardwareId;
use mutex::{BusMutex, RefCellBus};

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

pub trait SeesawDevice {
    type Error;
    type Driver: Driver;

    const DEFAULT_ADDR: u8;
    const HARDWARE_ID: HardwareId;
    const PRODUCT_ID: u16;

    fn addr(&self) -> u8;

    fn driver(&mut self) -> &mut Self::Driver;

    fn new(addr: u8, driver: Self::Driver) -> Self;

    fn new_with_default_addr(driver: Self::Driver) -> Self;
}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults.
pub trait SeesawDeviceInit<D: Driver>: SeesawDevice<Driver = D>
where
    Self: Sized,
{
    fn init(self) -> Result<Self, Self::Error>;
}
