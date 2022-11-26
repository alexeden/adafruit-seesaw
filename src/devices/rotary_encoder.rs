use super::{Addressable, SeesawDevice};
use crate::{
    bus::Attached,
    error::SeesawError,
    modules::{
        encoder::EncoderModule, gpio::GpioModule, neopixel::NeopixelModule, status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub struct RotaryEncoder<'a, B>(SevenBitAddress, &'a mut B);

impl<'a, B> Addressable for RotaryEncoder<'a, B> {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl<'a, B: crate::Bus> Attached<'a, B> for RotaryEncoder<'a, B> {
    fn bus(&'a mut self) -> &'a mut B {
        &mut self.1
    }
}

impl<'a, B: crate::Bus> GpioModule<'a, B> for RotaryEncoder<'a, B> {}
impl<'a, B: crate::Bus> EncoderModule<'a, B> for RotaryEncoder<'a, B> {}
impl<'a, B: crate::Bus> NeopixelModule<'a, B> for RotaryEncoder<'a, B> {
    const PIN: u8 = 6;
}

impl<'a, B: crate::Bus> SeesawDevice<'a, B> for RotaryEncoder<'a, B> {
    const DEFAULT_ADDR: u8 = 0x36;

    fn begin(bus: &'a mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>> {
        let mut device = RotaryEncoder(addr, bus);
        device.reset_and_begin()?;
        device.enable_button()?;
        device.enable_neopixel()?;
        Ok(device)
    }
}
