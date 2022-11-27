use super::{Addressable, Device};
use crate::{
    bus::{DelayBus, I2cBus},
    error::SeesawError,
    modules::StatusModule,
    SeesawBus, SeesawDriver,
};
use embedded_hal::blocking::{delay, i2c};
use shared_bus::BusMutex;

pub struct GenericDevice<M>(i2c::SevenBitAddress, M);

impl<I2C, DELAY> GenericDevice<SeesawBus<I2C, DELAY>>
where
    I2C: crate::I2cBus,
    DELAY: crate::DelayBus,
{
}

impl<M, I2C, DELAY> Device<M, I2C, DELAY> for GenericDevice<M>
where
    M: BusMutex<Bus = SeesawDriver<I2C, DELAY>>,
    DELAY: DelayBus,
    I2C: I2cBus,
{
    fn bus<'a>(&'a self) -> &'a M {
        &self.1
    }
}

impl<B> Addressable for GenericDevice<B> {
    fn addr(&self) -> i2c::SevenBitAddress {
        self.0
    }
}

// impl<B: crate::I2cBus + crate::DelayBus> Attached<B> for GenericDevice<B> {
//     fn bus(&mut self) -> &mut B {
//         &mut self.1
//     }
// }

impl<M, I2C, DELAY> StatusModule<M, I2C, DELAY> for GenericDevice<M>
where
    M: BusMutex<Bus = SeesawDriver<I2C, DELAY>>,
    DELAY: DelayBus,
    I2C: I2cBus,
{
}

// impl<B: crate::I2cBus + crate::DelayBus> SeesawDevice<B> for GenericDevice<B>
// {     const DEFAULT_ADDR: u8 = 0x30;

//     fn begin(bus: B, addr: i2c::SevenBitAddress) -> Result<Self,
// SeesawError<B::I2cError>> {         let mut device = GenericDevice(addr,
// bus);         device.reset_and_begin()?;
//         Ok(device)
//     }
// }
