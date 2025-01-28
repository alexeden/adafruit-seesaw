use super::SeesawDeviceInit;
use crate::{
    modules::{neopixel::NeopixelModule, status::StatusModule, HardwareId},
    seesaw_device, Driver,
};

seesaw_device! {
    name: NeoTrellis,
    hardware_id: HardwareId::SAMD09,
    product_id: 3954,
    default_addr: 0x2E,
    modules: [
        NeopixelModule { num_leds: 16, pin: 3 },
        KeypadModule { num_cols: 4, num_rows: 4 },
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoTrellis<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}
