use crate::{
    bus::BusRead,
    error::SeesawError,
    modules::{
        encoder::EncoderModule, gpio::GpioModule, neopixel::NeopixelModule, status::StatusModule,
    },
};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub trait Addressable {
    fn addr(&self) -> SevenBitAddress;
}

// All Seesaw devices support the Status module
impl<D: SeesawDevice> StatusModule for D {}

pub trait SeesawDevice: Addressable
where
    Self: Sized,
{
    const DEFAULT_ADDR: u8;

    fn begin<E, BUS: BusRead<E>>(
        bus: &mut BUS,
        addr: SevenBitAddress,
    ) -> Result<Self, SeesawError<E>>;

    fn begin_default<E, BUS: BusRead<E>>(bus: &mut BUS) -> Result<Self, SeesawError<E>> {
        Self::begin(bus, Self::DEFAULT_ADDR)
    }
}

pub struct RotaryEncoder(SevenBitAddress);
impl GpioModule for RotaryEncoder {}
impl EncoderModule for RotaryEncoder {}
impl NeopixelModule for RotaryEncoder {
    const PIN: u8 = 6;
}
impl Addressable for RotaryEncoder {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl SeesawDevice for RotaryEncoder {
    const DEFAULT_ADDR: u8 = 0x36;

    fn begin<E, BUS: BusRead<E>>(
        bus: &mut BUS,
        addr: SevenBitAddress,
    ) -> Result<Self, SeesawError<E>> {
        let mut device = RotaryEncoder(addr);
        device
            .reset_and_begin(bus)
            .and_then(|_| device.enable_neopixel(bus))
            .map(|_| device)
    }
}
