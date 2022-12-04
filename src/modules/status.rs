use crate::{driver::Driver, DriverExt, Modules, Reg, SeesawDevice};

const STATUS_HW_ID: &Reg = &[Modules::Status.into(), 0x01];
const STATUS_VERSION: &Reg = &[Modules::Status.into(), 0x02];
const STATUS_OPTIONS: &Reg = &[Modules::Status.into(), 0x03];
const STATUS_TEMP: &Reg = &[Modules::Status.into(), 0x04];
const STATUS_SWRST: &Reg = &[Modules::Status.into(), 0x7F];

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
            Ok(id) if id == hw_id => Ok(()),
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
            adc: value >> Modules::Adc as u8 & 1 == 1,
            dac: value >> Modules::Dac as u8 & 1 == 1,
            dap: value >> Modules::Dap as u8 & 1 == 1,
            eeprom: value >> Modules::Eeprom as u8 & 1 == 1,
            encoder: value >> Modules::Encoder as u8 & 1 == 1,
            gpio: value >> Modules::Gpio as u8 & 1 == 1,
            interrupt: value >> Modules::Interrupt as u8 & 1 == 1,
            keypad: value >> Modules::Keypad as u8 & 1 == 1,
            neopixel: value >> Modules::Neopixel as u8 & 1 == 1,
            sercom0: value >> Modules::Sercom0 as u8 & 1 == 1,
            spectrum: value >> Modules::Spectrum as u8 & 1 == 1,
            status: value >> Modules::Status as u8 & 1 == 1,
            timer: value >> Modules::Timer as u8 & 1 == 1,
            touch: value >> Modules::Touch as u8 & 1 == 1,
        }
    }
}

/// StatusModule
#[derive(Debug)]
pub struct ProductDateCode {
    pub id: u16,
    pub year: u8,
    pub month: u8,
    pub day: u8,
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
