use super::SeesawDeviceInit;
#[cfg(feature = "module_touch")]
use crate::modules::touch::TouchModule;
use crate::{
    modules::{status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device!(
    name: SoilSensor,
    hardware_id: HardwareId::SAMD09,
    product_id: 4026,
    default_addr: 0x36
);

#[cfg(feature = "module_touch")]
impl<D: Driver> TouchModule<D> for SoilSensor<D> {}

impl<D: Driver> SeesawDeviceInit<D> for SoilSensor<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw().map(|_| self)
    }
}
