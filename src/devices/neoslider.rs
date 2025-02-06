use super::SeesawDeviceInit;
use crate::{
    modules::{adc::AdcModule, neopixel::NeopixelModule, status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device!(
  /// NeoSlider
  name: NeoSlider,
  hardware_id: HardwareId::ATTINY817,
  product_id: 5295,
  default_addr: 0x30,
  modules: [
      AdcModule,
      GpioModule,
      NeopixelModule<color_type = NeoSliderColor> {  num_leds: 4, pin: 14 },
  ]
);

pub type NeoSliderColor = rgb::Grb<u8>;

impl<D: Driver> SeesawDeviceInit<D> for NeoSlider<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}

impl<D: Driver> NeoSlider<D> {
    pub fn slider_value(&mut self) -> Result<u16, SeesawError<D::Error>> {
        self.analog_read(18)
    }
}
