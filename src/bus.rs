use crate::driver::I2cDriver;
use embedded_hal::{delay, i2c};
use shared_bus::BusMutex;

#[derive(Debug)]
pub struct BusProxy<'a, M> {
    pub(crate) mutex: &'a M,
}

#[derive(Debug)]
pub struct Bus<DELAY, I2C>(pub(crate) DELAY, pub(crate) I2C);

// Clone implementation
impl<'a, DELAY, I2C, M> Clone for BusProxy<'a, M>
where
    DELAY: delay::DelayNs,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn clone(&self) -> Self {
        Self { mutex: self.mutex }
    }
}

// Delay implementation
impl<'a, DELAY, I2C, M> delay::DelayNs for BusProxy<'a, M>
where
    DELAY: delay::DelayNs,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn delay_ns(&mut self, ns: u32) {
        self.mutex.lock(|bus| bus.0.delay_ns(ns))
    }
}

// I2C implementations
impl<'a, DELAY, I2C, M> i2c::I2c for BusProxy<'a, M>
where
    DELAY: delay::DelayNs,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn transaction(
            &mut self,
            address: u8,
            operations: &mut [i2c::Operation<'_>],
        ) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.transaction(address, operations))
            .map_err(|err| err.into())
    }

    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.read(address, read))
            .map_err(|err| err.into())
    }

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.write(addr, buffer))
            .map_err(|err| err.into())
    }

    fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.write_read(address, write, read))
            .map_err(|err| err.into())
    }
}

impl<'a, DELAY, I2C, M> i2c::ErrorType for BusProxy<'a, M>
where
    DELAY: delay::DelayNs,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = I2C::Error;
}
