use crate::modules::Modules;
use embedded_hal::blocking::{delay, i2c};
// use embedded_hal::blocking::i2c;
// use shared_bus::BusMutex;
// mod generic_device;
// pub use generic_device::*;
use crate::{BusExt, Driver, SeesawError};
// use shared_bus::BusMutex;

pub trait Device<D: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>> {
    fn addr(&self) -> u8;

    fn create(addr: u8, driver: D) -> Self;

    fn driver<'a>(&'a mut self) -> &'a mut D;
}

#[derive(Debug)]
pub struct GenericDevice<M>(u8, M);

impl<D: Driver> Device<D> for GenericDevice<D> {
    fn addr(&self) -> u8 {
        self.0
    }

    fn create(addr: u8, driver: D) -> Self {
        Self(addr, driver)
    }

    fn driver<'a>(&'a mut self) -> &'a mut D {
        &mut self.1
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

impl<T, D> StatusModule<D> for T
where
    D: crate::Driver,
    T: Device<D>,
{
}

// impl<D: Driver> GenericDevice<D> {
// impl<D: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>>
// GenericDevice<D> {     // GenericDevice<D> {

//     pub fn new(addr: u8, driver: D) -> Self {
//         Self(addr, driver)
//     }
// }
