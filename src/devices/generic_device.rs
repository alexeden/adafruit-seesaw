use super::SeesawDeviceInit;
use crate::{
    modules::{status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  #[doc(hidden)]
  name: GenericDevice,
  hardware_id: HardwareId::SAMD09,
  product_id: 0,
  default_addr: 0x49
}

impl<D: Driver> SeesawDeviceInit<D> for GenericDevice<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset().map(|_| self)
    }
}
