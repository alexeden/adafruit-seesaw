#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
use embedded_hal::blocking::delay;
mod bus;
pub mod common;
pub(crate) mod device;
mod macros;
pub(crate) use device::*;
pub mod devices;
mod driver;
mod error;
pub mod modules;
pub use driver::*;
pub use error::*;
pub use modules::*;
pub mod prelude {
    pub use super::{driver::DriverExt, modules::*};
}

const DELAY_TIME: u32 = 125;

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

    pub fn connect<'a, D: DeviceInit<bus::BusProxy<'a, M>>>(
        &'a self,
        addr: u8,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        D::new(addr, driver).init()
    }

    pub fn connect_with<'a, D: Device<bus::BusProxy<'a, M>>, F: FnMut(D) -> Result<D, D::Error>>(
        &'a self,
        addr: u8,
        mut init: F,
    ) -> Result<D, D::Error> {
        let driver = bus::BusProxy { mutex: &self.mutex };
        let device = D::new(addr, driver);
        init(device)
    }
}

pub type SeesawSingleThread<BUS> = Seesaw<shared_bus::NullMutex<BUS>>;
