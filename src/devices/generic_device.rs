use super::{Addressable, Connect, Device};
use crate::{
    bus::{DelayBus, I2cBus},
    modules::StatusModule,
    SeesawBus, SeesawDriver, SeesawError,
};
use embedded_hal::blocking::i2c;
use shared_bus::BusMutex;

pub struct GenericDevice<M>(i2c::SevenBitAddress, M);

impl<D, M> Device<M::Bus, M> for GenericDevice<M>
where
    D: I2cBus + DelayBus,
    M: BusMutex<Bus = D>,
{
    fn bus<'a>(&'a self) -> &'a M {
        &self.1
    }
}

impl<B> Addressable for GenericDevice<B> {
    fn addr(&self) -> i2c::SevenBitAddress {
        self.0
    }
}

impl<D, M> StatusModule<M::Bus, M> for GenericDevice<M>
where
    D: I2cBus + DelayBus,
    M: BusMutex<Bus = D>,
{
}

impl<I2C, DELAY> Connect<I2C, DELAY> for GenericDevice<SeesawBus<I2C, DELAY>>
where
    DELAY: DelayBus,
    I2C: I2cBus,
{
    fn connect(
        i2c: I2C,
        delay: DELAY,
        addr: i2c::SevenBitAddress,
    ) -> Result<Self, SeesawError<I2C::I2cError>> {
        let device = Self(addr, SeesawBus::create(SeesawDriver::new(i2c, delay)));
        Ok(device)
    }
}
