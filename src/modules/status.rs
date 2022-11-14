use crate::{bus::BusRead, devices::Addressable, error::SeesawError};

pub trait StatusModule: Addressable {
    const MOD_REG: u8 = 0x00;
    const HARDWARE_ID_FN_REG: u8 = 0x01;
    const VERSION_FN_REG: u8 = 0x02;
    const OPTIONS_FN_REG: u8 = 0x03;
    const TEMP_FN_REG: u8 = 0x04;
    const RESET_FN_REG: u8 = 0x7F;

    fn hardware_id<E, BUS: BusRead<E>>(&self, bus: &mut BUS) -> Result<u8, SeesawError<E>> {
        bus.read_u8(self.addr(), Self::MOD_REG, Self::HARDWARE_ID_FN_REG)
    }

    fn options<E, BUS: BusRead<E>>(&self, bus: &mut BUS) -> Result<u32, SeesawError<E>> {
        bus.read_u32(self.addr(), Self::MOD_REG, Self::OPTIONS_FN_REG)
    }

    fn product_info<E, BUS: BusRead<E>>(
        &self,
        bus: &mut BUS,
    ) -> Result<ProductDateCode, SeesawError<E>> {
        bus.read_u32(self.addr(), Self::MOD_REG, Self::VERSION_FN_REG)
            .map(|version| version.into())
    }

    fn reset<E, BUS: BusRead<E>>(&self, bus: &mut BUS) -> Result<(), SeesawError<E>> {
        bus.write_u8(self.addr(), Self::MOD_REG, 0x7f, 0xFF)
            .map(|_| bus.delay_us(125_000))
    }

    fn temp<E, BUS: BusRead<E>>(&self, bus: &mut BUS) -> Result<f32, SeesawError<E>> {
        bus.read_u32(self.addr(), Self::MOD_REG, Self::TEMP_FN_REG)
            .map(|buf| (buf as f32 / (1u32 << 16) as f32))
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
