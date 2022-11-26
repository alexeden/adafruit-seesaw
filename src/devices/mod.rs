use crate::{bus::Attached, error::SeesawError, modules::status::StatusModule};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub mod neokey_1x4;
// pub mod neoslider;
// pub mod rotary_encoder;

pub trait Addressable {
    fn addr(&self) -> SevenBitAddress;
}

// All Seesaw devices support the Status module
impl<E, B: crate::Bus<E>, D: SeesawDevice<E, B>> StatusModule<E, B> for D {}

pub trait SeesawDevice<E, B: crate::Bus<E>>: Addressable + Attached<E, B>
where
    Self: Sized,
{
    const DEFAULT_ADDR: u8;

    fn begin(bus: B, addr: SevenBitAddress) -> Result<Self, SeesawError<E>>;

    fn begin_default(bus: B) -> Result<Self, SeesawError<E>> {
        Self::begin(bus, Self::DEFAULT_ADDR)
    }
}
