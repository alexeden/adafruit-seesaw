use super::SeesawDeviceInit;
#[cfg(feature = "module_neopixel")]
use crate::modules::neopixel::NeopixelModule;
use crate::{
    modules::{
        gpio::{GpioModule, PinMode},
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
  default_addr: 0x30
}

pub type NeoKey1x4Color = rgb::Grb<u8>;

impl<D: Driver> GpioModule<D> for NeoKey1x4<D> {}
#[cfg(feature = "module_neopixel")]
impl<D: Driver> NeopixelModule<D> for NeoKey1x4<D> {
    type Color = NeoKey1x4Color;

    const N_LEDS: usize = 4;
    const PIN: u8 = 3;
}

impl<D: Driver> SeesawDeviceInit<D> for NeoKey1x4<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        self.enable_button_pins()?;
        #[cfg(feature = "module_neopixel")]
        self.enable_neopixel()?;
        Ok(self)
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
