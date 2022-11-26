use super::{Addressable, SeesawDevice};
use crate::{
    bus::Attached,
    error::SeesawError,
    modules::{neopixel::NeopixelModule, status::StatusModule},
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub struct NeoSlider<'a, B>(SevenBitAddress, &'a mut B);

impl<'a, B: crate::Bus> Addressable for NeoSlider<'a, B> {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl<'a, B: crate::Bus> Attached<'a, B> for NeoSlider<'a, B> {
    fn bus(&'a mut self) -> &'a mut B {
        &mut self.1
    }
}

impl<'a, B: crate::Bus> NeopixelModule<'a, B> for NeoSlider<'a, B> {
    const N_LEDS: u16 = 4;
    const PIN: u8 = 14;
}

impl<'a, B: crate::Bus> SeesawDevice<'a, B> for NeoSlider<'a, B> {
    const DEFAULT_ADDR: u8 = 0x30;

    fn begin(bus: &mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>> {
        let mut device = NeoSlider(addr, bus);
        device.reset_and_begin()?;
        device.enable_neopixel()?;
        Ok(device)
    }
}
