#![no_std]
#![forbid(unsafe_code)]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
// TODO improve the organization of the exports/visibility
use embedded_hal::blocking::delay;
mod bus;
mod common;
pub mod device;
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

const DELAY_TIME: u32 = 12500;
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

    pub fn connect_default_addr<'a, D: device::DeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let mut device = D::new(D::DEFAULT_ADDR, driver);
        device.init().map(|_| device)
    }

    pub fn connect<'a, D: device::DeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
        addr: u8,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let mut device = D::new(addr, driver);
        device.init().map(|_| device)
    }

    pub fn connect_with<
        'a,
        D: device::Device<bus::BusProxy<'a, M>>,
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

/// All devices implement the status module
impl<D: Driver, T: device::Device<D>> StatusModule<D> for T {}
