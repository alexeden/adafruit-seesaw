use super::SeesawDeviceInit;
#[cfg(feature = "module_neopixel")]
use crate::modules::neopixel::NeopixelModule;
use crate::{
    modules::{adc::AdcModule, gpio::GpioModule, status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device!(
  /// NeoSlider
  name: NeoSlider,
  hardware_id: HardwareId::ATTINY817,
  product_id: 5295,
  default_addr: 0x30
);

pub type NeoSliderColor = rgb::Grb<u8>;

impl<D: Driver> AdcModule<D> for NeoSlider<D> {}
impl<D: Driver> GpioModule<D> for NeoSlider<D> {}
#[cfg(feature = "module_neopixel")]
impl<D: Driver> NeopixelModule<D> for NeoSlider<D> {
    type Color = NeoSliderColor;

    const N_LEDS: usize = 4;
    const PIN: u8 = 14;
}

impl<D: Driver> SeesawDeviceInit<D> for NeoSlider<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        #[cfg(feature = "module_neopixel")]
        self.enable_neopixel()?;
        Ok(self)
    }
}

impl<D: Driver> NeoSlider<D> {
    pub fn slider_value(&mut self) -> Result<u16, SeesawError<D::Error>> {
        self.analog_read(18)
    }
}
