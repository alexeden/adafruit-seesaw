use crate::{device::DeviceInit, seesaw_device, StatusModule};

seesaw_device!(GenericDevice);

impl<D: crate::Driver> DeviceInit<D> for GenericDevice<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}
