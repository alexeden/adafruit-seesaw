use super::SeesawDeviceInit;
use crate::{
    modules::{
        encoder::{EncoderConfig, EncoderModule},
        gpio::GpioConfig,
        neopixel::{NeopixelConfig, NeopixelModule},
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

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

impl<D> GpioConfig for NeoRotary4<D> {}
impl<D> EncoderConfig<4> for NeoRotary4<D> {
    const ENCODER_BTN_PINS: [u8; 4] = [12, 14, 17, 9];
}

impl<D> NeopixelConfig for NeoRotary4<D> {
    type Color = NeoRotary4Color;

    const N_LEDS: usize = 4;
    const PIN: u8 = 18;
}

impl<D: Driver> SeesawDeviceInit<D> for NeoRotary4<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button(0))
            .and_then(|_| self.enable_button(1))
            .and_then(|_| self.enable_button(2))
            .and_then(|_| self.enable_button(3))
            .map(|_| self)
    }
}
