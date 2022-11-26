#![no_std]
#![allow(dead_code, incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs)]
use bus::Bus;
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{SevenBitAddress, Write, WriteRead},
};
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
    DELAY: DelayUs<u32>,
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        SeesawBus { bus, delay }
    }
}

impl<I2C, DELAY> DelayUs<u32> for SeesawBus<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us)
    }
}

impl<I2C, DELAY, E> Bus<E> for SeesawBus<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    fn write<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), SeesawError<E>>
    where
        [(); N + 2]: Sized,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0..2].copy_from_slice(reg);
        buffer[2..].copy_from_slice(bytes);

        self.bus
            .write(addr, &buffer)
            .map(|_| self.delay_us(DELAY_TIME))
            .map_err(SeesawError::I2c)
    }

    fn read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], SeesawError<E>> {
        let mut buffer = [0u8; N];
        self.bus
            .write(addr, reg)
            .and_then(|_| {
                self.delay_us(DELAY_TIME);
                self.bus.write_read(addr, &[], &mut buffer)
            })
            .map_err(SeesawError::I2c)
            .map(|_| buffer)
    }
}
