use super::SeesawDeviceInit;
use crate::{
    modules::{
	status::StatusModule,
	quad_encoder::QuadEncoderModule,
	HardwareId,
    },
    seesaw_device, Driver,
};

seesaw_device! {
    name: NeoRotary4,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5752,
    default_addr: 0x49,
    modules: [
	QuadEncoderModule,
	GpioModule,
	NeopixelModule { num_leds: 4, pin: 18 }
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoRotary4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
	self.reset_and_verify_seesaw()
	    .and_then(|_| self.enable_button(0))
	    .and_then(|_| self.enable_button(1))
	    .and_then(|_| self.enable_button(2))
	    .and_then(|_| self.enable_button(3))
	    .map(|_| self)
    }
}
