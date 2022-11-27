#![no_std]
#![allow(dead_code, incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs)]
use bus::{DelayBus, I2cBus, I2cExt};
use core::cell;
use embedded_hal::blocking::{delay, i2c};
pub mod bus;
pub use error::SeesawError;
use modules::Reg;
pub mod devices;
pub mod error;
mod modules;

// Exports
// pub use devices::SeesawDevice;
pub use modules::*;

const DELAY_TIME: u32 = 125;

#[derive(Debug)]
pub struct Seesaw<M>(M);

#[derive(Debug)]
pub struct SeesawBus<I2C, DELAY> {
    bus: cell::RefCell<SeesawDriver<I2C, DELAY>>,
}

#[derive(Debug)]
pub struct SeesawDriver<I2C, DELAY>(I2C, DELAY);

impl<I2C, DELAY> shared_bus::BusMutex for SeesawBus<I2C, DELAY>
where
    DELAY: DelayBus,
    I2C: I2cBus,
{
    type Bus = SeesawDriver<I2C, DELAY>;

    fn create(v: Self::Bus) -> Self {
        Self {
            bus: cell::RefCell::new(v),
        }
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let mut bus = self.bus.borrow_mut();
        f(&mut bus)
    }
}

impl<I2C, DELAY> SeesawBus<I2C, DELAY>
where
    DELAY: delay::DelayUs<u32>,
    I2C: i2c::WriteRead + i2c::Write + i2c::Read,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        Self {
            bus: cell::RefCell::new(SeesawDriver(bus, delay)),
        }
    }
}

impl<I2C, DELAY> delay::DelayUs<u32> for SeesawDriver<I2C, DELAY>
where
    DELAY: delay::DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.1.delay_us(us)
    }
}

impl<I2C, DELAY> i2c::Write for SeesawDriver<I2C, DELAY>
where
    I2C: i2c::Write,
{
    type Error = <I2C as i2c::Write>::Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write(address, bytes)
    }
}

impl<I2C, DELAY> i2c::WriteRead for SeesawDriver<I2C, DELAY>
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
        self.0.write_read(address, bytes, buffer)
    }
}

impl<I2C, DELAY> i2c::Read for SeesawDriver<I2C, DELAY>
where
    I2C: i2c::Read,
{
    type Error = <I2C as i2c::Read>::Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.0.read(address, buffer)
    }
}

// impl<I2C: I2cBus, DELAY: DelayBus> I2cExt for SeesawDriver<I2C, DELAY> {
//     type Error = I2C::I2cError;

//     fn register_read<const N: usize>(
//         &mut self,
//         addr: i2c::SevenBitAddress,
//         reg: &crate::Reg,
//     ) -> Result<[u8; N], Self::Error> {
//         let mut buffer = [0u8; N];
//         self.write(addr, reg)?;
//         self.delay_us(crate::DELAY_TIME);
//         self.write_read(addr, &[], &mut buffer)?;
//         Ok(buffer)
//     }

//     fn register_write<const N: usize>(
//         &mut self,
//         addr: i2c::SevenBitAddress,
//         reg: &crate::Reg,
//         bytes: &[u8; N],
//     ) -> Result<(), Self::Error>
//     where
//         [(); N + 2]: Sized,
//     {
//         let mut buffer = [0u8; N + 2];
//         buffer[0..2].copy_from_slice(reg);
//         buffer[2..].copy_from_slice(bytes);

//         self.write(addr, &buffer)?;
//         self.delay_us(crate::DELAY_TIME);
//         Ok(())
//     }
// }
