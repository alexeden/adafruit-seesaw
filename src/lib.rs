#![no_std]
#![forbid(unsafe_code)]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
use embedded_hal::blocking::delay;
mod bus;
mod common;
pub mod devices;
mod driver;
mod macros;
pub mod modules;
pub use common::*;
pub use devices::*;
pub use driver::*;
pub use modules::*;

pub mod prelude {
    pub use super::{devices::*, driver::DriverExt, modules::*};
}

const DELAY_TIME: u32 = 125;
pub type SeesawSingleThread<BUS> = Seesaw<shared_bus::NullMutex<BUS>>;

pub struct Seesaw<M> {
    mutex: M,
}

impl<DELAY, I2C, M> Seesaw<M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: shared_bus::BusMutex<Bus = bus::Bus<DELAY, I2C>>,
{
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Seesaw {
            mutex: M::create(bus::Bus(delay, i2c)),
        }
    }

    pub fn connect_default_addr<'a, D: SeesawDeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let mut device = D::new(D::DEFAULT_ADDR, driver);
        device.init().map(|_| device)
    }

    pub fn connect<'a, D: SeesawDeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
        addr: u8,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let mut device = D::new(addr, driver);
        device.init().map(|_| device)
    }

    pub fn connect_with<
        'a,
        D: SeesawDevice<bus::BusProxy<'a, M>>,
        F: FnMut(&mut D) -> Result<(), D::Error>,
    >(
        &'a self,
        addr: u8,
        mut init: F,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let mut device = D::new(addr, driver);
        init(&mut device).map(|_| device)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),
    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}

pub trait SeesawDevice<D: Driver> {
    type Error;

    const DEFAULT_ADDR: u8;
    const HARDWARE_ID: u8;
    const PRODUCT_ID: u16;

    fn addr(&self) -> u8;

    fn driver(&mut self) -> &mut D;

    fn new(addr: u8, driver: D) -> Self;
}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we're need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults. You can
/// override the default initialization function with your own by calling
/// `Seesaw::connect_with` instead of `Seesaw::connect`.
pub trait SeesawDeviceInit<D: Driver>: SeesawDevice<D>
where
    Self: Sized,
{
    fn init(&mut self) -> Result<(), Self::Error>;
}
