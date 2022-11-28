#![no_std]
#![allow(unused_imports, incomplete_features, const_evaluatable_unchecked)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
mod bus;
pub mod devices;
mod driver;
mod error;
pub mod modules;
pub use bus::*;
use devices::{Device, GenericDevice};
use driver::DriverProxy;
use embedded_hal::blocking::{delay, i2c};
pub use error::SeesawError;
use shared_bus::{BusMutex, NullMutex};

pub mod prelude {
    pub use super::bus::BusExt;
}

const DELAY_TIME: u32 = 125;

#[derive(Debug)]
pub struct Seesaw<M> {
    mutex: M,
}

impl<M: BusMutex> Seesaw<M>
where
    M::Bus: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>,
{
    pub fn new(bus: M::Bus) -> Self {
        Seesaw {
            mutex: M::create(bus),
        }
    }

    fn driver<'a>(&'a self) -> DriverProxy<'a, M> {
        DriverProxy { mutex: &self.mutex }
    }

    pub fn connect<'a, D: Device<DriverProxy<'a, M>>>(&'a self, addr: u8) -> D {
        D::begin(addr, self.driver())
    }
}

pub type SeesawSingleThread<BUS> = Seesaw<NullMutex<BUS>>;

#[derive(Debug)]
pub struct SeesawBus<DELAY, I2C>(DELAY, I2C);

impl<DELAY, I2C: I2cBus> SeesawBus<DELAY, I2C>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::Write + i2c::WriteRead + i2c::Read,
{
    pub fn new(delay: DELAY, i2c: I2C) -> Self {
        Self(delay, i2c)
    }
}

impl<DELAY, I2C> delay::DelayUs<u32> for SeesawBus<DELAY, I2C>
where
    DELAY: delay::DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us)
    }
}

impl<DELAY, I2C> i2c::Write for SeesawBus<DELAY, I2C>
where
    I2C: i2c::Write,
{
    type Error = <I2C as i2c::Write>::Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.1.write(address, bytes)
    }
}

impl<DELAY, I2C> i2c::WriteRead for SeesawBus<DELAY, I2C>
where
    I2C: i2c::WriteRead,
{
    type Error = <I2C as i2c::WriteRead>::Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.1.write_read(address, bytes, buffer)
    }
}

impl<DELAY, I2C> i2c::Read for SeesawBus<DELAY, I2C>
where
    I2C: i2c::Read,
{
    type Error = <I2C as i2c::Read>::Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.1.read(address, buffer)
    }
}
