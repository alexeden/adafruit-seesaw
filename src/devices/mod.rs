use core::borrow::BorrowMut;

use crate::modules::Modules;
use embedded_hal::blocking::{delay, i2c};
// use embedded_hal::blocking::i2c;
// use shared_bus::BusMutex;
// mod generic_device;
// pub use generic_device::*;
use crate::{driver::DriverProxy, BusExt, Driver, SeesawError};
use shared_bus::BusMutex;
// use shared_bus::BusMutex;

// pub trait Device {
pub trait Device<D: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>> {
    fn addr(&self) -> u8;

    fn driver<'a>(&'a mut self) -> &'a mut D;

    fn begin(addr: u8, driver: D) -> Self;
}

#[derive(Debug)]
pub struct GenericDevice<M>(u8, M);

impl<D: Driver> Device<D> for GenericDevice<D> {
    fn addr(&self) -> u8 {
        self.0
    }

    fn begin(addr: u8, driver: D) -> Self {
        Self(addr, driver)
    }

    fn driver<'a>(&'a mut self) -> &'a mut D {
        self.1.borrow_mut()
    }
}

pub trait StatusModule<D>: Device<D>
where
    D: crate::Driver,
{
    fn hardware_id(&mut self) -> Result<u8, SeesawError<<D>::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_u8(addr, &[Modules::Status.into(), 0x01])
            .map_err(SeesawError::I2c)
    }
}

// impl<D: Driver> GenericDevice<D> {
// impl<D: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>>
// GenericDevice<D> {     // GenericDevice<D> {

//     pub fn new(addr: u8, driver: D) -> Self {
//         Self(addr, driver)
//     }
// }
