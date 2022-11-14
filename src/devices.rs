use crate::{
    bus::BusRead,
    error::SeesawError,
    modules::{gpio::GpioModule, status::StatusModule},
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
    fn begin<E, BUS: BusRead<E>>(
        bus: &mut BUS,
        addr: SevenBitAddress,
    ) -> Result<Self, SeesawError<E>>;
}

pub struct RotaryEncoder(SevenBitAddress);
impl GpioModule for RotaryEncoder {}
impl Addressable for RotaryEncoder {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}

impl SeesawDevice for RotaryEncoder {
    fn begin<E, BUS: BusRead<E>>(
        bus: &mut BUS,
        addr: SevenBitAddress,
    ) -> Result<Self, SeesawError<E>> {
        let mut device = RotaryEncoder(addr);
        device.reset_and_begin(bus).map(|_| device)
    }
}
