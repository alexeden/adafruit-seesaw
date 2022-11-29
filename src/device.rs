use crate::driver::Driver;

pub trait Device<D: Driver> {
    type Error;

    const DEFAULT_ADDR: u8;
    const PRODUCT_ID: u16;

    fn addr(&self) -> u8;

    fn driver(&mut self) -> &mut D;

    fn new(addr: u8, driver: D) -> Self;
}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we're need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults. You can
/// override the default initialization function with your own by calling
/// `Seesaw::connect_with` instead of `Seesaw::connect`.
pub trait DeviceInit<D: Driver>: Device<D>
where
    Self: Sized,
{
    fn init(&mut self) -> Result<(), Self::Error>;
}
