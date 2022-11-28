#![no_std]
#![allow(const_evaluatable_unchecked, incomplete_features)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
use embedded_hal::blocking::{delay, i2c};
use shared_bus::{BusMutex, NullMutex};
mod bus;
pub(crate) mod device;
pub(crate) use device::*;
pub mod devices;
mod driver;
mod error;
pub mod modules;
use bus::{Bus, BusProxy};
pub use driver::*;
pub use error::SeesawError;
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
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Seesaw {
            mutex: M::create(Bus(delay, i2c)),
        }
    }

    pub fn connect<'a, D: Connect<BusProxy<'a, M>>>(&'a self, addr: u8) -> Result<D, D::Error> {
        let driver = BusProxy { mutex: &self.mutex };
        D::new(addr, driver).connect()
    }

    pub fn connect_with<'a, D: Device<BusProxy<'a, M>>, E, F: FnMut(D) -> Result<D, E>>(
        &'a self,
        addr: u8,
        mut f: F,
    ) -> Result<D, E> {
        let driver = BusProxy { mutex: &self.mutex };
        let device = D::new(addr, driver);
        f(device)
    }
}

pub type SeesawSingleThread<BUS> = Seesaw<NullMutex<BUS>>;
