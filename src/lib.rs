#![no_std]
#![allow(dead_code, incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs)]
use embedded_hal::blocking::{delay, i2c};
pub mod bus;
use error::SeesawError;
use modules::Reg;
pub mod devices;
pub mod error;
pub mod modules;

const DELAY_TIME: u32 = 125;

#[derive(Debug)]
pub struct SeesawBus<I2C, DELAY> {
    bus: I2C,
    delay: DELAY,
}

impl<I2C, DELAY, E> SeesawBus<I2C, DELAY>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        SeesawBus { bus, delay }
    }
}

impl<I2C, DELAY> delay::DelayUs<u32> for SeesawBus<I2C, DELAY>
where
    DELAY: delay::DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us)
    }
}

impl<I2C, DELAY> i2c::Write for SeesawBus<I2C, DELAY>
where
    I2C: i2c::Write,
{
    type Error = <I2C as i2c::Write>::Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.bus.write(address, bytes)
    }
}

impl<I2C, DELAY> i2c::WriteRead for SeesawBus<I2C, DELAY>
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
        self.bus.write_read(address, bytes, buffer)
    }
}
