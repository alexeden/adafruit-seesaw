#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(array_try_map, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
use embedded_hal::delay;
pub mod bus;
mod common;
pub mod devices;
mod driver;
mod macros;
pub mod modules;
pub use common::*;
pub use devices::*;
pub use driver::*;

pub mod prelude {
    pub use super::{
        devices::*,
        driver::DriverExt,
        modules::{adc::*, encoder::*, gpio::*, neopixel::*, status::*, timer::*},
        SeesawDevice, SeesawDeviceInit,
    };
}

pub type SeesawSingleThread<BUS> = Seesaw<shared_bus::NullMutex<BUS>>;

pub struct Seesaw<M> {
    mutex: M,
}

impl<DELAY, I2C, M> Seesaw<M>
where
    DELAY: delay::DelayNs,
    I2C: I2cDriver,
    M: shared_bus::BusMutex<Bus = bus::Bus<DELAY, I2C>>,
{
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Seesaw {
            mutex: M::create(bus::Bus(delay, i2c)),
        }
    }

    pub fn acquire_driver(&self) -> bus::BusProxy<'_, M> {
        bus::BusProxy { mutex: &self.mutex }
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
/// calls to be made. e.g. for a Neokey1x4, we're need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults. You can
/// override the default initialization function with your own by calling
/// `Seesaw::connect_with` instead of `Seesaw::connect`.
pub trait SeesawDeviceInit<D: Driver>: SeesawDevice<Driver = D>
where
    Self: Sized,
{
    fn init(self) -> Result<Self, Self::Error>;
}
