// use embedded_hal::blocking::i2c;
// use shared_bus::BusMutex;
// mod generic_device;
// pub use generic_device::*;

use crate::Driver;
// use shared_bus::BusMutex;

pub trait Device {
    fn addr(&self) -> u8;

    fn begin<D: Driver>(addr: u8, driver: D) -> Self;
}

// pub trait Connect<I2C: crate::I2cBus, DELAY: crate::DelayBus>
// where
//     Self: Sized,
// {
//     fn connect(
//         i2c: I2C,
//         delay: DELAY,
//         addr: i2c::SevenBitAddress,
//     ) -> Result<Self, crate::SeesawError<I2C::I2cError>>;
// }

// pub trait SeesawDevice<D, M>
// where
//     M: BusMutex<Bus = D>,
//     D: crate::Driver,
// {
//     fn addr(&self) -> u8;
//     fn bus<'a>(&'a self) -> &'a M;
// }

// impl<SSD, D, M> StatusModule<D, M> for SSD
// where
//     SSD: SeesawDevice<D, M>,
//     //  + Connect<I2C, DELAY>,
//     // Self: Driver,
//     D: crate::Driver,
//     M: BusMutex<Bus = D>,
//     // I2C: I2cBus,
//     // DELAY: DelayBus,
// {
// }
