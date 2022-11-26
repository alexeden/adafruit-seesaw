use super::{Addressable, SeesawDevice};
use crate::{
    bus::Attached,
    error::SeesawError,
    modules::{
        gpio::{GpioModule, PinMode},
        neopixel::NeopixelModule,
        status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

const NEOKEY_1X4_PINMASK: u32 = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

pub struct NeoKey1x4<'a, B>(SevenBitAddress, &'a mut B);

impl<'a, B> Addressable for NeoKey1x4<'a, B> {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl<'a, B: crate::Bus> Attached<'a, B> for NeoKey1x4<'a, B> {
    fn bus(&'a mut self) -> &'a mut B {
        &mut self.1
    }
}

impl<'a, B: crate::Bus> GpioModule<'a, B> for NeoKey1x4<'a, B> {}
impl<'a, B: crate::Bus> NeopixelModule<'a, B> for NeoKey1x4<'a, B> {
    const N_LEDS: u16 = 4;
    const PIN: u8 = 3;
}

impl<'a, B: crate::Bus> SeesawDevice<'a, B> for NeoKey1x4<'a, B> {
    const DEFAULT_ADDR: u8 = 0x30;

    fn begin(bus: &'a mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>> {
        let mut device = NeoKey1x4(addr, bus);
        device
            .reset_and_begin()
            .and_then(|_| device.enable_neopixel())
            .and_then(|_| device.set_pin_mode_bulk(NEOKEY_1X4_PINMASK, PinMode::InputPullup))
            .map(|_| device)
    }
}

// Additional methods
impl<'a, B: crate::Bus> NeoKey1x4<'a, B>
where
    Self: GpioModule<'a, B>,
{
    pub fn keys(&mut self) -> Result<u8, SeesawError<B::I2cError>> {
        self.digital_read_bulk().map(|r| (r >> 4 & 0xF) as u8)
    }

    pub fn keys_bool(&mut self) -> Result<[bool; 4], SeesawError<B::I2cError>> {
        self.keys().map(|b| {
            [
                0 == 1 & b >> 0,
                0 == 1 & b >> 1,
                0 == 1 & b >> 2,
                0 == 1 & b >> 3,
            ]
        })
    }
}
