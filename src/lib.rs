#![no_std]
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{Write, WriteRead},
};

pub struct Seesaw<I2C, DELAY> {
    bus: I2C,
    delay: DELAY,
}

impl<I2C, DELAY> DelayUs<u32> for Seesaw<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us)
    }
}

impl<I2C, DELAY, E> Seesaw<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        Seesaw { bus, delay }
    }
}
