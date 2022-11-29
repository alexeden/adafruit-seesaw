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

    pub fn connect_default_addr<'a, D: device::DeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        D::new(D::DEFAULT_ADDR, driver).init()
    }

    pub fn connect<'a, D: device::DeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
        addr: u8,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        D::new(addr, driver).init()
    }

    pub fn connect_with<
        'a,
        D: device::Device<bus::BusProxy<'a, M>>,
        F: FnMut(D) -> Result<D, D::Error>,
    >(
        &'a self,
        addr: u8,
        mut init: F,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let device = D::new(addr, driver);
        init(device)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SeesawError<E> {
    /// I2C bus error
    I2c(E),

    /// Occurs when an invalid hardware ID is read
    InvalidHardwareId(u8),
}
