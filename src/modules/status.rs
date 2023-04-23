use crate::{driver::Driver, DriverExt, MODULE_STATUS, MODULE_GPIO, MODULE_SERCOM0, MODULE_TIMER, MODULE_ADC, MODULE_DAC, MODULE_INTERRUPT, MODULE_DAP, MODULE_EEPROM, MODULE_NEOPIXEL, MODULE_TOUCH, MODULE_KEYPAD, MODULE_ENCODER, MODULE_SPECTRUM, Reg, SeesawDevice};

const STATUS_HW_ID: &Reg = &[MODULE_STATUS, 0x01];
const STATUS_VERSION: &Reg = &[MODULE_STATUS, 0x02];
const STATUS_OPTIONS: &Reg = &[MODULE_STATUS, 0x03];
const STATUS_TEMP: &Reg = &[MODULE_STATUS, 0x04];
const STATUS_SWRST: &Reg = &[MODULE_STATUS, 0x7F];

pub trait StatusModule<D: Driver>: SeesawDevice<Driver = D> {
    fn capabilities(&mut self) -> Result<DeviceCapabilities, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_OPTIONS)
            .map(|opts| opts.into())
            .map_err(crate::SeesawError::I2c)
    }

    fn hardware_id(&mut self) -> Result<u8, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_u8(addr, STATUS_HW_ID)
            .map_err(crate::SeesawError::I2c)
    }

    fn product_info(&mut self) -> Result<ProductDateCode, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_VERSION)
            .map(|version| version.into())
            .map_err(crate::SeesawError::I2c)
    }

    fn reset(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .write_u8(addr, STATUS_SWRST, 0xFF)
            .map(|_| self.driver().delay_us(125_000))
            .map_err(crate::SeesawError::I2c)
    }

    fn reset_and_verify_seesaw(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let hw_id = Self::HARDWARE_ID;
        self.reset().and_then(|_| match self.hardware_id() {
            Ok(id) if id == hw_id.into() => Ok(()),
            Ok(id) => Err(crate::SeesawError::InvalidHardwareId(id)),
            Err(e) => Err(e),
        })
    }

    fn temp(&mut self) -> Result<f32, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_TEMP)
            .map(|buf| (buf as f32 / (1u32 << 16) as f32))
            .map_err(crate::SeesawError::I2c)
    }
}

/// StatusModule
#[derive(Copy, Clone, Debug)]
pub struct DeviceCapabilities {
    pub adc: bool,
    pub dac: bool,
    pub dap: bool,
    pub eeprom: bool,
    pub encoder: bool,
    pub gpio: bool,
    pub interrupt: bool,
    pub keypad: bool,
    pub neopixel: bool,
    pub sercom0: bool,
    pub spectrum: bool,
    pub status: bool,
    pub timer: bool,
    pub touch: bool,
}

impl From<u32> for DeviceCapabilities {
    fn from(value: u32) -> Self {
        DeviceCapabilities {
            adc: value >> MODULE_ADC as u8 & 1 == 1,
            dac: value >> MODULE_DAC as u8 & 1 == 1,
            dap: value >> MODULE_DAP as u8 & 1 == 1,
            eeprom: value >> MODULE_EEPROM as u8 & 1 == 1,
            encoder: value >> MODULE_ENCODER as u8 & 1 == 1,
            gpio: value >> MODULE_GPIO as u8 & 1 == 1,
            interrupt: value >> MODULE_INTERRUPT as u8 & 1 == 1,
            keypad: value >> MODULE_KEYPAD as u8 & 1 == 1,
            neopixel: value >> MODULE_NEOPIXEL as u8 & 1 == 1,
            sercom0: value >> MODULE_SERCOM0 as u8 & 1 == 1,
            spectrum: value >> MODULE_SPECTRUM as u8 & 1 == 1,
            status: value >> MODULE_STATUS as u8 & 1 == 1,
            timer: value >> MODULE_TIMER as u8 & 1 == 1,
            touch: value >> MODULE_TOUCH as u8 & 1 == 1,
        }
    }
}

/// StatusModule
#[derive(Debug)]
pub struct ProductDateCode {
    pub id: u16,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<u32> for ProductDateCode {
    fn from(vers: u32) -> Self {
        Self {
            id: (vers >> 16) as u16,
            year: ((vers & 0x3F) + 2000) as u16,
            month: ((vers >> 7) & 0xF) as u8,
            day: ((vers >> 11) & 0x1F) as u8,
        }
    }
}
