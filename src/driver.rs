use embedded_hal::blocking::{delay, i2c};
use shared_bus::BusMutex;

#[derive(Debug)]
pub struct DriverProxy<'a, M> {
    pub(crate) mutex: &'a M,
}

// Clone implementation
impl<'a, M: BusMutex> Clone for DriverProxy<'a, M> {
    fn clone(&self) -> Self {
        Self { mutex: &self.mutex }
    }
}

// Delay implementation
impl<'a, M: BusMutex> delay::DelayUs<u32> for DriverProxy<'a, M>
where
    M::Bus: delay::DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.mutex.lock(|bus| bus.delay_us(us))
    }
}

// I2C implementations
impl<'a, M: BusMutex> i2c::Write for DriverProxy<'a, M>
where
    M::Bus: i2c::Write,
{
    type Error = <M::Bus as i2c::Write>::Error;

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write(addr, buffer))
    }
}

impl<'a, M: BusMutex> i2c::Read for DriverProxy<'a, M>
where
    M::Bus: i2c::Read,
{
    type Error = <M::Bus as i2c::Read>::Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.read(addr, buffer))
    }
}

impl<'a, M: BusMutex> i2c::WriteRead for DriverProxy<'a, M>
where
    M::Bus: i2c::WriteRead,
{
    type Error = <M::Bus as i2c::WriteRead>::Error;

    fn write_read(
        &mut self,
        addr: u8,
        buffer_in: &[u8],
        buffer_out: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.write_read(addr, buffer_in, buffer_out))
    }
}
