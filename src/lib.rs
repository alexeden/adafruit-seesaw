#![no_std]
#![allow(dead_code, incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs)]
use bus::{BusRead, BusWrite};
/// Try making the bus a distinct thing instead of the parent wrapper around dev
/// ices e.g. each _device_ has a `SeesawBus` (maybe mutexed?)
/// There's no reason for the bus (current called `Seesaw`) struct to own
/// all the device/board stuff as part of its type
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{SevenBitAddress, Write, WriteRead},
};
pub(crate) mod bus;
use error::SeesawError;
use modules::Reg;
pub mod devices;
pub mod error;
pub mod modules;

pub struct SeesawBus<I2C, DELAY> {
    bus: I2C,
    delay: DELAY,
    delay_time: u32,
}

impl<I2C, DELAY, E> SeesawBus<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        SeesawBus {
            bus,
            delay,
            delay_time: 125,
        }
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

impl<I2C, DELAY, E> BusWrite<E> for SeesawBus<I2C, DELAY>
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
            .map(|_| self.delay_us(self.delay_time))
            .map_err(SeesawError::I2c)
    }
}

impl<I2C, DELAY, E> BusRead<E> for SeesawBus<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    fn read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], SeesawError<E>> {
        let mut buffer = [0u8; N];
        self.bus
            .write(addr, reg)
            .and_then(|_| {
                self.delay_us(self.delay_time);
                self.bus.write_read(addr, &[], &mut buffer)
            })
            .map_err(SeesawError::I2c)
            .map(|_| buffer)
    }
}
