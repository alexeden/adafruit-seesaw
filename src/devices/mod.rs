use crate::{
    bus::{DelayBus, I2cBus},
    SeesawError,
};
use embedded_hal::blocking::i2c;
mod generic_device;
pub use generic_device::*;
use shared_bus::BusMutex;

pub trait Addressable {
    fn addr(&self) -> i2c::SevenBitAddress;
}

pub trait Device<D, M>: Addressable
where
    M: BusMutex<Bus = D>,
    D: I2cBus + DelayBus,
{
    fn bus<'a>(&'a self) -> &'a M;
}

pub trait Connect<I2C: crate::I2cBus, DELAY: crate::DelayBus>
where
    Self: Sized,
{
    fn connect(
        i2c: I2C,
        delay: DELAY,
        addr: i2c::SevenBitAddress,
    ) -> Result<Self, SeesawError<I2C::I2cError>>;
}
