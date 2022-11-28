use crate::{device::Connect, seesaw_device};
use embedded_hal::blocking::{delay, i2c};

seesaw_device!(GenericDevice, StatusModule);

impl<D, E> Connect<D, E> for GenericDevice<D>
where
    D: crate::Driver,
{
    fn connect(self) -> Result<Self, crate::SeesawError<E>> {
        todo!()
    }
}
