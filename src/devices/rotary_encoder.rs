use super::SeesawDeviceInit;
use crate::{
    modules::{encoder::EncoderModule, neopixel::NeopixelModule, status::StatusModule, HardwareId},
    seesaw_device, Driver,
};

seesaw_device! {
  /// RotaryEncoder
  name: RotaryEncoder,
  hardware_id: HardwareId::SAMD09,
  product_id: 4991,
  default_addr: 0x36,
  modules:  [
      EncoderModule { num_encoders: 1, encoder_btn_pins: [24] },
      GpioModule,
      NeopixelModule { num_leds: 1, pin: 6 },
  ]
}

impl<D: Driver> SeesawDeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_button(0))
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}
