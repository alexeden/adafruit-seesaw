use super::SeesawDeviceInit;
#[cfg(feature = "module_neopixel")]
use crate::modules::neopixel::NeopixelModule;
use crate::{
    modules::{encoder::EncoderModule, gpio::GpioModule, status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  name: RotaryEncoder,
  hardware_id: HardwareId::SAMD09,
  product_id: 4991,
  default_addr: 0x36
}

pub type RotaryEncoderColor = rgb::Grb<u8>;

impl<D: Driver> GpioModule<D> for RotaryEncoder<D> {}
impl<D: Driver> EncoderModule<D, 1> for RotaryEncoder<D> {
    const ENCODER_BTN_PINS: [u8; 1] = [24];
}

#[cfg(feature = "module_neopixel")]
impl<D: Driver> NeopixelModule<D> for RotaryEncoder<D> {
    type Color = RotaryEncoderColor;

    const N_LEDS: usize = 1;
    const PIN: u8 = 6;
}

impl<D: Driver> SeesawDeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        self.enable_button(0)?;
        #[cfg(feature = "module_neopixel")]
        self.enable_neopixel()?;
        Ok(self)
    }
}
