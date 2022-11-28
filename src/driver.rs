use crate::Driver;
use embedded_hal::blocking::{delay, i2c};
use shared_bus::BusMutex;

#[derive(Debug)]
pub struct DriverProxy<'a, M> {
    pub(crate) mutex: &'a M,
}

#[derive(Debug)]
pub struct Bus<DELAY, I2C>(DELAY, I2C);

// Clone implementation
impl<'a, DELAY, I2C, M> Clone for DriverProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn clone(&self) -> Self {
        Self { mutex: &self.mutex }
    }
}

// Delay implementation
impl<'a, DELAY, I2C, M> delay::DelayUs<u32> for DriverProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    fn delay_us(&mut self, us: u32) {
        self.mutex.lock(|bus| bus.0.delay_us(us))
    }
}

// I2C implementations
impl<'a, DELAY, I2C, M> i2c::Write for DriverProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = <I2C as i2c::Write>::Error;

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.1.write(addr, buffer))
    }
}

impl<'a, DELAY, I2C, M> i2c::Read for DriverProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = <I2C as i2c::Read>::Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.1.read(addr, buffer))
    }
}

impl<'a, DELAY, I2C, M> i2c::WriteRead for DriverProxy<'a, M>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,

    M: BusMutex<Bus = Bus<DELAY, I2C>>,
{
    type Error = <I2C as i2c::WriteRead>::Error;

    fn write_read(
        &mut self,
        addr: u8,
        buffer_in: &[u8],
        buffer_out: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.1.write_read(addr, buffer_in, buffer_out))
    }
}
