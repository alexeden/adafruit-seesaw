#![no_std]
#![allow(incomplete_features, const_evaluatable_unchecked)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
mod bus;
pub mod devices;
mod driver;
mod error;
pub mod modules;
use bus::{Bus, BusProxy};
use devices::Device;
pub use driver::*;
use embedded_hal::blocking::{delay, i2c};
pub use error::SeesawError;
use shared_bus::{BusMutex, NullMutex};

pub mod prelude {
    pub use super::driver::DriverExt;
}

const DELAY_TIME: u32 = 125;

#[derive(Debug)]
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

    fn driver<'a>(&'a self) -> BusProxy<'a, M> {
        BusProxy { mutex: &self.mutex }
    }

    pub fn connect<'a, D: Device<BusProxy<'a, M>>>(&'a self, addr: u8) -> D {
        D::create(addr, self.driver())
    }
}

pub type SeesawSingleThread<BUS> = Seesaw<NullMutex<BUS>>;
