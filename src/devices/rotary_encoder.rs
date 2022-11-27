use super::{Addressable, SeesawDevice};
use crate::{
    bus::Attached,
    error::SeesawError,
    modules::{
        encoder::EncoderModule, gpio::GpioModule, neopixel::NeopixelModule, status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub struct RotaryEncoder<B>(SevenBitAddress, B);

impl<B> Addressable for RotaryEncoder<B> {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl<B: crate::Bus> Attached<B> for RotaryEncoder<B> {
    fn bus(&mut self) -> &mut B {
        &mut self.1
    }
}

impl<B: crate::Bus> GpioModule<B> for RotaryEncoder<B> {}
impl<B: crate::Bus> EncoderModule<B> for RotaryEncoder<B> {}
impl<B: crate::Bus> NeopixelModule<B> for RotaryEncoder<B> {
    const PIN: u8 = 6;
}

impl<B: crate::Bus> SeesawDevice<B> for RotaryEncoder<B> {
    const DEFAULT_ADDR: u8 = 0x36;

    fn begin(bus: B, addr: SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>> {
        let mut device = RotaryEncoder(addr, bus);
        device.reset_and_begin()?;
        device.enable_button()?;
        device.enable_neopixel()?;
        Ok(device)
    }
}
