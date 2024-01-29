use crate::mutex::BusMutex;
use embedded_hal::{
    delay::DelayNs,
    i2c::{self, ErrorType, I2c},
};

#[derive(Debug)]
pub struct Bus<'a, M>(pub(crate) &'a M);

impl<'a, DELAY, I2C, M> Clone for Bus<'a, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

// Delay implementation
impl<'a, DELAY, I2C, M> DelayNs for Bus<'a, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    fn delay_ns(&mut self, ns: u32) {
        self.0.lock(|bus| bus.0.delay_ns(ns))
    }
}

impl<'a, DELAY, I2C, M> ErrorType for Bus<'a, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    type Error = I2C::Error;
}

impl<'a, DELAY, I2C, M> I2c for Bus<'a, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.0
            .lock(|bus| bus.1.transaction(address, operations))
            .map_err(|err| err.into())
    }
}
