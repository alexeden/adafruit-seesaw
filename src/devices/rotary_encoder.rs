use super::{Addressable, SeesawDevice};
use crate::{
    bus::Bus,
    error::SeesawError,
    modules::{
        encoder::EncoderModule, gpio::GpioModule, neopixel::NeopixelModule, status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub struct RotaryEncoder(SevenBitAddress);
impl Addressable for RotaryEncoder {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}
impl GpioModule for RotaryEncoder {}
impl EncoderModule for RotaryEncoder {}
impl NeopixelModule for RotaryEncoder {
    const PIN: u8 = 6;
}

impl SeesawDevice for RotaryEncoder {
    const DEFAULT_ADDR: u8 = 0x36;

    fn begin<E, B: Bus<E>>(bus: &mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<E>> {
        let mut device = RotaryEncoder(addr);
        device
            .reset_and_begin(bus)
            .and_then(|_| device.enable_button(bus))
            .and_then(|_| device.enable_neopixel(bus))
            .map(|_| device)
    }
}
