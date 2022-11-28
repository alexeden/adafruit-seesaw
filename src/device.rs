pub trait Device<D: crate::Driver> {
    const DEFAULT_ADDR: u8;

    type Error;

    fn addr(&self) -> u8;

    fn driver<'a>(&'a mut self) -> &'a mut D;

    fn new(addr: u8, driver: D) -> Self;
}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we're need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults. You can
/// override the default initialization function with your own by calling
/// `Seesaw::connect_with` instead of `Seesaw::connect`.
pub trait DeviceInit<D: crate::Driver>: Device<D>
where
    Self: Sized,
{
    fn init(self) -> Result<Self, Self::Error>;
}

/// All devices implement the status module
impl<D: crate::Driver, T: Device<D>> crate::StatusModule<D> for T {}

#[macro_export]
macro_rules! seesaw_device {
    ($device:ident) => {
        /// $device
        #[derive(Debug)]
        pub struct $device<M>(u8, M);

        impl<D: crate::Driver> $crate::device::Device<D> for $device<D> {
            type Error = $crate::SeesawError<D::I2cError>;

            fn addr(&self) -> u8 {
                self.0
            }

            fn driver<'a>(&'a mut self) -> &'a mut D {
                &mut self.1
            }

            fn new(addr: u8, driver: D) -> Self {
                Self(addr, driver)
            }
        }
    };
}
