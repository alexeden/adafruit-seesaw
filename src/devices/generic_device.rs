use crate::{device::Connect, seesaw_device, StatusModule};

seesaw_device!(GenericDevice, StatusModule);

impl<D> Connect<D> for GenericDevice<D>
where
    D: crate::Driver,
{
    type Error = crate::SeesawError<D::I2cError>;

    fn connect(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}
