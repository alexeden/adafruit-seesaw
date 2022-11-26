use crate::{
    bus::{Attached, Bus},
    error::SeesawError,
    modules::status::StatusModule,
};
use embedded_hal::blocking::i2c::SevenBitAddress;

// pub mod neokey_1x4;
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

    fn begin(addr: SevenBitAddress) -> Result<Self, SeesawError<E>>;

    fn begin_default() -> Result<Self, SeesawError<E>> {
        Self::begin(Self::DEFAULT_ADDR)
    }
}
