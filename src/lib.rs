#![no_std]
#![allow(dead_code, incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs)]
/// Try making the bus a distinct thing instead of the parent wrapper around dev
/// ices e.g. each _device_ has a `SeesawBus` (maybe mutexed?)
/// There's no reason for the bus (current called `Seesaw`) struct to own
/// all the device/board stuff as part of its type
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{SevenBitAddress, Write, WriteRead},
};
use error::SeesawError;
use regs::{Readable, Writable};
pub mod error;
pub mod modules;
mod regs;

pub struct SeesawBus<I2C, DELAY> {
    bus: I2C,
    delay: DELAY,
    delay_time: u32,
}

impl<I2C, DELAY> DelayUs<u32> for SeesawBus<I2C, DELAY>
where
    DELAY: DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us)
    }
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

    fn read<R, const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
    ) -> Result<[u8; N], SeesawError<E>>
    where
        R: Readable + Writable,
    {
        let mut buffer = [0u8; N];
        self.bus
            .write(addr, &[reg.module(), reg.function()])
            .and_then(|_| {
                self.delay_us(self.delay_time);
                self.bus.write_read(addr, &[], &mut buffer)
            })
            .map_err(SeesawError::I2c)
            .map(|_| buffer)
    }

    fn read_u8<R: Readable + Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
    ) -> Result<u8, SeesawError<E>> {
        self.read::<_, 1>(addr, reg).map(|buf| buf[0])
    }

    fn read_i32<R: Readable + Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
    ) -> Result<i32, SeesawError<E>> {
        self.read::<_, 4>(addr, reg).map(i32::from_be_bytes)
    }

    fn read_u16<R: Readable + Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
    ) -> Result<u16, SeesawError<E>> {
        self.read::<_, 2>(addr, reg).map(u16::from_be_bytes)
    }

    fn read_u32<R: Readable + Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
    ) -> Result<u32, SeesawError<E>> {
        self.read::<_, 4>(addr, reg).map(u32::from_be_bytes)
    }

    fn write<R, const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
        bytes: &[u8; N],
    ) -> Result<(), SeesawError<E>>
    where
        [(); N + 2]: Sized,
        R: Writable,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0] = reg.module();
        buffer[1] = reg.function();
        buffer[2..].copy_from_slice(bytes);

        self.bus
            .write(addr, &buffer)
            .map(|_| self.delay_us(self.delay_time))
            .map_err(SeesawError::I2c)
    }

    fn write_u8<R: Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
        value: u8,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &[value])
    }

    fn write_u16<R: Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
        value: u16,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &u16::to_be_bytes(value))
    }

    fn write_i32<R: Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
        value: i32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &i32::to_be_bytes(value))
    }

    fn write_u32<R: Writable>(
        &mut self,
        addr: SevenBitAddress,
        reg: R,
        value: u32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &u32::to_be_bytes(value))
    }
}
