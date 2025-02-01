use super::SeesawDeviceInit;
use crate::{
    modules::{
        gpio::{GpioModule, PinMode},
        neopixel::{NeopixelModule, RGB},
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  /// NeoKey1x4
  name: NeoKey1x4,
  hardware_id: HardwareId::SAMD09,
  product_id: 4980,
  default_addr: 0x30,
  modules: [
      GpioModule,
      NeopixelModule<RGB> { num_leds: 4, pin: 3 },
  ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoKey1x4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button_pins())
            .map(|_| self)
    }
}

impl<D: Driver> NeoKey1x4<D> {
    pub fn enable_button_pins(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode_bulk(
            (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7),
            PinMode::InputPullup,
        )
    }

    pub fn keys(&mut self) -> Result<u8, SeesawError<D::Error>> {
        self.digital_read_bulk().map(|r| ((r >> 4) & 0xF) as u8)
    }
}
