use crate::driver::I2cDriver;
use embedded_hal::blocking::{delay, i2c};
use shared_bus::BusMutex;

#[derive(Debug)]
pub struct BusProxy<'a, M> {
    pub(crate) mutex: &'a M,
}

#[derive(Debug)]
pub(crate) struct Bus<DELAY, I2C>(pub(crate) DELAY, pub(crate) I2C);

// Clone implementation
impl<'a, DELAY, I2C, M> Clone for BusProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn clone(&self) -> Self {
        Self { mutex: &self.mutex }
    }
}

// Delay implementation
impl<'a, DELAY, I2C, M> delay::DelayUs<u32> for BusProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn delay_us(&mut self, us: u32) {
        self.mutex.lock(|bus| bus.0.delay_us(us))
    }
}

// I2C implementations
impl<'a, DELAY, I2C, M> i2c::Write for BusProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = I2C::I2cError;

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.write(addr, buffer))
            .map_err(|err| err.into())
    }
}

impl<'a, DELAY, I2C, M> i2c::Read for BusProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = I2C::I2cError;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.read(addr, buffer))
            .map_err(|err| err.into())
    }
}

impl<'a, DELAY, I2C, M> i2c::WriteRead for BusProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: I2cDriver,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = I2C::I2cError;

    fn write_read(
        &mut self,
        addr: u8,
        buffer_in: &[u8],
        buffer_out: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.write_read(addr, buffer_in, buffer_out))
            .map_err(|err| err.into())
    }
}
