use crate::{
    device::DeviceInit, driver::Driver, impl_device_encoder_module, impl_device_gpio_module,
    impl_device_neopixel_module, modules::*, seesaw_device,
};

seesaw_device!(
  /// GenericDevice
  GenericDevice,
  default_addr: 0,
  product_id: 0
);

impl<D: Driver> DeviceInit<D> for GenericDevice<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}

seesaw_device!(
  /// NeoKey1x4
  NeoKey1x4,
  default_addr: 0x30,
  product_id: 4980
);
impl_device_gpio_module!(NeoKey1x4);
impl_device_neopixel_module!(NeoKey1x4, num_leds: 4, pin: 3);

impl<D: Driver> DeviceInit<D> for NeoKey1x4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button_pins())
            .map(|_| self)
    }
}

impl<D: Driver> NeoKey1x4<D> {
    pub fn enable_button_pins(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_pin_mode_bulk(
            (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7),
            PinMode::InputPullup,
        )
    }

    pub fn keys(&mut self) -> Result<u8, crate::SeesawError<D::I2cError>> {
        self.digital_read_bulk().map(|r| (r >> 4 & 0xF) as u8)
    }
}

seesaw_device!(
  /// NeoSlider
  NeoSlider,
  default_addr: 0x30,
  product_id: 5295
);
impl_device_gpio_module!(NeoSlider);
impl_device_neopixel_module!(NeoSlider, num_leds: 4, pin: 14);

impl<D: Driver> DeviceInit<D> for NeoSlider<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}

seesaw_device!(
  /// RotaryEncoder
  RotaryEncoder,
  default_addr: 0x36,
  product_id: 4991
);
impl_device_encoder_module!(RotaryEncoder, button_pin: 24);
impl_device_gpio_module!(RotaryEncoder);
impl_device_neopixel_module!(RotaryEncoder, num_leds: 1, pin: 6);

impl<D: Driver> DeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_button())
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}
