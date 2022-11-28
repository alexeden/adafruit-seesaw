use embedded_hal::blocking::i2c;
use shared_bus::BusMutex;
mod generic_device;
pub use generic_device::*;

pub trait Connect<I2C: crate::I2cBus, DELAY: crate::DelayBus>
where
    Self: Sized,
{
    fn connect(
        i2c: I2C,
        delay: DELAY,
        addr: i2c::SevenBitAddress,
    ) -> Result<Self, crate::SeesawError<I2C::I2cError>>;
}

pub trait SeesawDevice<D, M>
where
    M: BusMutex<Bus = D>,
    D: crate::Driver,
{
    fn addr(&self) -> u8;
    fn bus<'a>(&'a self) -> &'a M;
}
