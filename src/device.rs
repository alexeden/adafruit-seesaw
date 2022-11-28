use embedded_hal::blocking::{delay, i2c};

pub trait Device<D: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32>> {
    fn addr(&self) -> u8;

    fn create(addr: u8, driver: D) -> Self;

    fn driver<'a>(&'a mut self) -> &'a mut D;
}

impl<T, D> crate::StatusModule<D> for T
where
    D: crate::Driver,
    T: Device<D>,
{
}

#[macro_export]
macro_rules! seesaw_device {
    ($device:ident, $( $x:ty ),*) => {
        #[derive(Debug)]
        pub struct $device<M>(u8, M);

        impl<D: crate::Driver> $crate::device::Device<D> for $device<D> {
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
    };
}
