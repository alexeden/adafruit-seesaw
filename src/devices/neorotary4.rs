use super::SeesawDeviceInit;
use crate::{
    modules::{encoder::EncoderModule, neopixel::NeopixelModule, status::StatusModule, HardwareId},
    seesaw_device, Driver,
};

seesaw_device! {
    /// Anecdotally, I've had a lot of issues with the quad rotary encoder.
    ///
    /// Specifically, calls to set/reset the encoders' position seem to have no
    /// effect on the firmware's internal position counters.
    name: NeoRotary4,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5752,
    default_addr: 0x49,
    modules: [
        EncoderModule { num_encoders: 4, encoder_btn_pins: [12, 14, 17, 9] },
        GpioModule,
        NeopixelModule<color_type = rgb::Rgb<u8>> { num_leds: 4, pin: 18 }
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoRotary4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button(0))
            .and_then(|_| self.enable_button(1))
            .and_then(|_| self.enable_button(2))
            .and_then(|_| self.enable_button(3))
            .map(|_| self)
    }
}
