use crate::{error::SeesawError, regs::Reg, SeesawBus};
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{SevenBitAddress, Write, WriteRead},
};

const MODULE_ID: u8 = 0x00;
const HARDWARE_ID: Reg<MODULE_ID, 0x01> = Reg;
const VERSION: Reg<MODULE_ID, 0x02> = Reg;
const OPTIONS: Reg<MODULE_ID, 0x03> = Reg;
const TEMP: Reg<MODULE_ID, 0x04> = Reg;
const RESET: Reg<MODULE_ID, 0x7F> = Reg;

pub trait Addressable {
    // const ADDR: SevenBitAddress;
    fn addr(&self) -> SevenBitAddress;
}

pub trait StatusModule: Addressable {
    fn hardware_id<E, I2C, DELAY>(
        &self,
        bus: &mut SeesawBus<I2C, DELAY>,
    ) -> Result<u8, SeesawError<E>>
    where
        DELAY: DelayUs<u32>,
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        bus.read_u8(self.addr(), HARDWARE_ID)
    }

    fn options<E, I2C, DELAY>(&self, bus: &mut SeesawBus<I2C, DELAY>) -> Result<u32, SeesawError<E>>
    where
        DELAY: DelayUs<u32>,
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        bus.read_u32(self.addr(), OPTIONS)
    }

    fn product_info<E, I2C, DELAY>(
        &self,
        bus: &mut SeesawBus<I2C, DELAY>,
    ) -> Result<ProductDateCode, SeesawError<E>>
    where
        DELAY: DelayUs<u32>,
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        bus.read_u32(self.addr(), VERSION)
            .map(|version| version.into())
    }

    fn reset<E, I2C, DELAY>(&self, bus: &mut SeesawBus<I2C, DELAY>) -> Result<(), SeesawError<E>>
    where
        DELAY: DelayUs<u32>,
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        bus.write_u8(self.addr(), RESET, 0xFF)
    }
}

// All Seesaw devices support the Status module
impl<A: Addressable> StatusModule for A {}

#[derive(Debug)]
pub struct ProductDateCode {
    id: u16,
    year: u8,
    month: u8,
    day: u8,
}

impl From<u32> for ProductDateCode {
    fn from(vers: u32) -> Self {
        Self {
            id: (vers >> 16) as u16,
            year: (vers & 0x3F) as u8,
            month: ((vers >> 7) & 0xF) as u8,
            day: ((vers >> 11) & 0x1F) as u8,
        }
    }
}

pub struct RotaryEncoder(pub SevenBitAddress);
impl Addressable for RotaryEncoder {
    fn addr(&self) -> SevenBitAddress {
        self.0
    }
}
