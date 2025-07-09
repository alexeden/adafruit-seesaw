use super::SeesawDeviceInit;
use crate::{
    modules::{encoder::EncoderModule, status::StatusModule, HardwareId},
    prelude::GpioModule,
    seesaw_device, Driver, SeesawError,
};
#[cfg(feature = "module_neopixel")]
use crate::modules::neopixel::NeopixelModule;

seesaw_device! {
    /// Anecdotally, I've had a lot of issues with the quad rotary encoder.
    ///
    /// Specifically, calls to set/reset the encoders' position seem to have no
    /// effect on the firmware's internal position counters.
    name: NeoRotary4,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5752,
    default_addr: 0x49
}

pub type NeoRotary4Color = rgb::Grb<u8>;

impl<D: Driver> GpioModule<D> for NeoRotary4<D> {}
impl<D: Driver> EncoderModule<D, 4> for NeoRotary4<D> {
    const ENCODER_BTN_PINS: [u8; 4] = [12, 14, 17, 9];
}
#[cfg(feature = "module_neopixel")]
impl<D: Driver> NeopixelModule<D> for NeoRotary4<D> {
    type Color = NeoRotary4Color;

    const N_LEDS: usize = 4;
    const PIN: u8 = 18;
}

impl<D: Driver> SeesawDeviceInit<D> for NeoRotary4<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        self.enable_button(0)?;
        self.enable_button(1)?;
        self.enable_button(2)?;
        self.enable_button(3)?;
        #[cfg(feature = "module_neopixel")]
        self.enable_neopixel()?;
        Ok(self)
    }
}
