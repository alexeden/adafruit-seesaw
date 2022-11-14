use embedded_hal::blocking::i2c::SevenBitAddress;

pub trait Addressable {
    fn addr(&self) -> SevenBitAddress;
}

pub struct RotaryEncoder(pub SevenBitAddress);
impl Addressable for RotaryEncoder {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}
