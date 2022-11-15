use super::{Addressable, SeesawDevice};
use crate::{
    bus::Bus,
    error::SeesawError,
    modules::{neopixel::NeopixelModule, status::StatusModule},
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub struct NeoSlider(SevenBitAddress);
impl Addressable for NeoSlider {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}
impl NeopixelModule for NeoSlider {
    const N_LEDS: u16 = 4;
    const PIN: u8 = 14;
}

impl SeesawDevice for NeoSlider {
    const DEFAULT_ADDR: u8 = 0x30;

    fn begin<E, B: Bus<E>>(bus: &mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<E>> {
        let mut device = NeoSlider(addr);
        device
            .reset_and_begin(bus)
            .and_then(|_| device.enable_neopixel(bus))
            .map(|_| device)
    }
}
