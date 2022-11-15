use crate::{bus::Bus, error::SeesawError, modules::status::StatusModule};
use embedded_hal::blocking::i2c::SevenBitAddress;

pub mod neokey_1x4;
pub mod neoslider;
pub mod rotary_encoder;

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

    fn begin<E, B: Bus<E>>(bus: &mut B, addr: SevenBitAddress) -> Result<Self, SeesawError<E>>;

    fn begin_default<E, B: Bus<E>>(bus: &mut B) -> Result<Self, SeesawError<E>> {
        Self::begin(bus, Self::DEFAULT_ADDR)
    }
}
