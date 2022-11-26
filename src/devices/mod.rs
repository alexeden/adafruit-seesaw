use crate::{bus::Attached, modules::status::StatusModule, SeesawError};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub mod neokey_1x4;
pub mod neoslider;
pub mod rotary_encoder;

pub trait Addressable {
    fn addr(&self) -> SevenBitAddress;
}

// All Seesaw devices support the Status module
impl<'a, B: crate::Bus, D: SeesawDevice<'a, B>> StatusModule<'a, B> for D {}

pub trait SeesawDevice<'a, B: crate::Bus>: Addressable + Attached<'a, B>
where
    Self: Sized,
{
    const DEFAULT_ADDR: u8;

    fn begin(bus: &'a mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>>;

    fn begin_default(bus: &'a mut B) -> Result<Self, SeesawError<B::I2cError>> {
        Self::begin(bus, Self::DEFAULT_ADDR)
    }
}
