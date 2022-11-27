use crate::{
    bus::{DelayBus, I2cBus, I2cExt},
    SeesawBus, SeesawDriver, SeesawError,
};
use embedded_hal::blocking::i2c;
mod generic_device;
pub use generic_device::*;
use shared_bus::BusMutex;

pub trait Addressable {
    fn addr(&self) -> i2c::SevenBitAddress;
}
// pub trait Attached<B: I2cExt> {
//     fn bus(&mut self) -> &mut B;
// }

pub trait Device<M, I2C, DELAY>: Addressable
where
    M: BusMutex<Bus = SeesawDriver<I2C, DELAY>>,
    DELAY: DelayBus,
    I2C: I2cBus,
{
    fn bus<'a>(&'a self) -> &'a M;
}

// pub trait SeesawDevice<B: crate::I2cBus + crate::DelayBus>: Addressable +
// Attached<B> where
//     Self: Sized,
// {
//     const DEFAULT_ADDR: u8;

//     fn begin(bus: B, addr: i2c::SevenBitAddress) -> Result<Self,
// SeesawError<B::I2cError>>; }

pub trait Connect<I2C: crate::I2cBus, DELAY: crate::DelayBus> {
    fn connect(i2c: I2C, delay: DELAY) -> Self;
}
