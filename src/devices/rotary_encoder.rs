use super::SeesawDeviceInit;
use crate::{
    modules::{
        encoder::{EncoderConfig, EncoderModule},
        gpio::GpioModule,
        neopixel::{NeopixelConfig, NeopixelModule},
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  name: RotaryEncoder,
  hardware_id: HardwareId::SAMD09,
  product_id: 4991,
  default_addr: 0x36,
  modules:  [ ]
}

pub type RotaryEncoderColor = rgb::Grb<u8>;

impl<D: Driver> GpioModule<D> for RotaryEncoder<D> {}

impl<D: Driver> EncoderConfig<1> for RotaryEncoder<D> {
    const ENCODER_BTN_PINS: [u8; 1] = [24];
}

impl<D> NeopixelConfig for RotaryEncoder<D> {
    type Color = RotaryEncoderColor;

    const N_LEDS: usize = 1;
    const PIN: u8 = 6;
}

impl<D: Driver> SeesawDeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_button(0))
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}
