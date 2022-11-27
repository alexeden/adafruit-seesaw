use crate::{bus::Attached, modules::status::StatusModule, SeesawError};
use embedded_hal::blocking::i2c;

pub mod neokey_1x4;
pub mod neoslider;
pub mod rotary_encoder;

pub trait Addressable {
    fn addr(&self) -> i2c::SevenBitAddress;
}

// All Seesaw devices support the Status module
impl<B: crate::Bus, D: SeesawDevice<B>> StatusModule<B> for D {}

pub trait SeesawDevice<B: crate::Bus>: Addressable + Attached<B>
where
    Self: Sized,
{
    const DEFAULT_ADDR: u8;

    fn begin(bus: B, addr: i2c::SevenBitAddress) -> Result<Self, SeesawError<B::I2cError>>;

    fn begin_default(bus: B) -> Result<Self, SeesawError<B::I2cError>> {
        Self::begin(bus, Self::DEFAULT_ADDR)
    }
}
