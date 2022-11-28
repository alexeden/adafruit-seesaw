#![no_std]
#![allow(incomplete_features, const_evaluatable_unchecked)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
use core::cell;
use embedded_hal::blocking::{delay, i2c};
pub mod bus;
pub mod devices;
pub mod error;
mod modules;
pub(crate) use bus::*;
pub use error::SeesawError;
use modules::Reg;

// Exports
pub use modules::*;

const DELAY_TIME: u32 = 125;

#[derive(Debug)]
pub struct SeesawBus<I2C, DELAY> {
    bus: cell::RefCell<SeesawDriver<I2C, DELAY>>,
}

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
    DELAY: DelayBus,
    I2C: I2cBus,
{
    pub fn new(bus: I2C, delay: DELAY) -> Self {
        Self {
            bus: cell::RefCell::new(SeesawDriver(bus, delay)),
        }
    }
}

#[derive(Debug)]
pub struct SeesawDriver<I2C, DELAY>(I2C, DELAY);

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
