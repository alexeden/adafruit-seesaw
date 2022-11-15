use super::{
    Reg, ADC_MODULE_ID, DAC_MODULE_ID, DAP_MODULE_ID, EEPROM_MODULE_ID, ENCODER_MODULE_ID,
    GPIO_MODULE_ID, INTERRUPT_MODULE_ID, KEYPAD_MODULE_ID, NEOPIXEL_MODULE_ID, SEESAW_HW_ID,
    SERCOM0_MODULE_ID, SPECTRUM_MODULE_ID, STATUS_MODULE_ID, TOUCH_MODULE_ID,
};
use crate::{bus::Bus, devices::Addressable, error::SeesawError};
const STATUS_HW_ID: &Reg = &[STATUS_MODULE_ID, 0x01];
const STATUS_VERSION: &Reg = &[STATUS_MODULE_ID, 0x02];
const STATUS_OPTIONS: &Reg = &[STATUS_MODULE_ID, 0x03];
const STATUS_TEMP: &Reg = &[STATUS_MODULE_ID, 0x04];
const STATUS_SWRST: &Reg = &[STATUS_MODULE_ID, 0x7F];

pub trait StatusModule: Addressable {
    fn reset_and_begin<E, B: Bus<E>>(&mut self, bus: &mut B) -> Result<(), SeesawError<E>> {
        self.reset(bus).and_then(|_| {
            bus.delay_us(12_500);
            match self.hardware_id(bus) {
                Ok(SEESAW_HW_ID) => Ok(()),
                Ok(id) => Err(SeesawError::InvalidHardwareId(id)),
                Err(e) => Err(e),
            }
        })
    }

    fn capabilities<E, B: Bus<E>>(
        &self,
        bus: &mut B,
    ) -> Result<DeviceCapabilities, SeesawError<E>> {
        bus.read_u32(self.addr(), STATUS_OPTIONS)
            .map(|opts| opts.into())
    }

    fn hardware_id<E, B: Bus<E>>(&self, bus: &mut B) -> Result<u8, SeesawError<E>> {
        bus.read_u8(self.addr(), STATUS_HW_ID)
    }

    fn product_info<E, B: Bus<E>>(&self, bus: &mut B) -> Result<ProductDateCode, SeesawError<E>> {
        bus.read_u32(self.addr(), STATUS_VERSION)
            .map(|version| version.into())
    }

    fn reset<E, B: Bus<E>>(&self, bus: &mut B) -> Result<(), SeesawError<E>> {
        bus.write_u8(self.addr(), STATUS_SWRST, 0xFF)
            .map(|_| bus.delay_us(125_000))
    }

    fn temp<E, B: Bus<E>>(&self, bus: &mut B) -> Result<f32, SeesawError<E>> {
        bus.read_u32(self.addr(), STATUS_TEMP)
            .map(|buf| (buf as f32 / (1u32 << 16) as f32))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DeviceCapabilities {
    adc: bool,
    dac: bool,
    dap: bool,
    eeprom: bool,
    encoder: bool,
    gpio: bool,
    interrupt: bool,
    keypad: bool,
    neopixel: bool,
    sercom0: bool,
    spectrum: bool,
    status: bool,
    timer: bool,
    touch: bool,
}

impl From<u32> for DeviceCapabilities {
    fn from(value: u32) -> Self {
        DeviceCapabilities {
            adc: value >> ADC_MODULE_ID & 1 == 1,
            dac: value >> DAC_MODULE_ID & 1 == 1,
            dap: value >> DAP_MODULE_ID & 1 == 1,
            eeprom: value >> EEPROM_MODULE_ID & 1 == 1,
            encoder: value >> ENCODER_MODULE_ID & 1 == 1,
            gpio: value >> GPIO_MODULE_ID & 1 == 1,
            interrupt: value >> INTERRUPT_MODULE_ID & 1 == 1,
            keypad: value >> KEYPAD_MODULE_ID & 1 == 1,
            neopixel: value >> NEOPIXEL_MODULE_ID & 1 == 1,
            sercom0: value >> SERCOM0_MODULE_ID & 1 == 1,
            spectrum: value >> SPECTRUM_MODULE_ID & 1 == 1,
            status: value >> STATUS_MODULE_ID & 1 == 1,
            timer: value >> STATUS_MODULE_ID & 1 == 1,
            touch: value >> TOUCH_MODULE_ID & 1 == 1,
        }
    }
}

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
