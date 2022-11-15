use super::{Addressable, SeesawDevice};
use crate::{
    bus::Bus,
    error::SeesawError,
    modules::{
        gpio::{GpioModule, PinMode},
        neopixel::NeopixelModule,
        status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

const NEOKEY_1X4_PINMASK: u32 = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

pub struct NeoKey1x4(SevenBitAddress);
impl Addressable for NeoKey1x4 {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl GpioModule for NeoKey1x4 {}
impl NeopixelModule for NeoKey1x4 {
    const N_LEDS: u16 = 4;
    const PIN: u8 = 3;
}

impl SeesawDevice for NeoKey1x4 {
    const DEFAULT_ADDR: u8 = 0x36;

    fn begin<E, B: Bus<E>>(bus: &mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<E>> {
        let mut device = NeoKey1x4(addr);
        device
            .reset_and_begin(bus)
            .and_then(|_| device.enable_neopixel(bus))
            .and_then(|_| device.set_pin_mode_bulk(bus, NEOKEY_1X4_PINMASK, PinMode::InputPullup))
            .map(|_| device)
    }
}
