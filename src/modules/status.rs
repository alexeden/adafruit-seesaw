use super::{
    Reg, ADC_MODULE_ID, DAC_MODULE_ID, DAP_MODULE_ID, EEPROM_MODULE_ID, ENCODER_MODULE_ID,
    GPIO_MODULE_ID, INTERRUPT_MODULE_ID, KEYPAD_MODULE_ID, NEOPIXEL_MODULE_ID, SEESAW_HW_ID,
    SERCOM0_MODULE_ID, SPECTRUM_MODULE_ID, STATUS_MODULE_ID, TOUCH_MODULE_ID,
};
use crate::{
    bus::{DelayBus, I2cBus, I2cExt},
    devices::{Addressable, Device},
    error::SeesawError,
};
use shared_bus::BusMutex;

const STATUS_HW_ID: &Reg = &[STATUS_MODULE_ID, 0x01];
const STATUS_VERSION: &Reg = &[STATUS_MODULE_ID, 0x02];
const STATUS_OPTIONS: &Reg = &[STATUS_MODULE_ID, 0x03];
const STATUS_TEMP: &Reg = &[STATUS_MODULE_ID, 0x04];
const STATUS_SWRST: &Reg = &[STATUS_MODULE_ID, 0x7F];

pub trait StatusModule<D, M>: Addressable + Device<D, M>
where
    M: BusMutex<Bus = D>,
    D: I2cBus + DelayBus,
{
    fn reset_and_begin(&mut self) -> Result<(), SeesawError<<D as I2cBus>::I2cError>> {
        self.reset()?;
        self.bus().lock(|driver| {
            driver.delay_us(12_500);
        });
        match self.hardware_id() {
            Ok(SEESAW_HW_ID) => Ok(()),
            Ok(id) => Err(SeesawError::InvalidHardwareId(id)),
            Err(e) => Err(e),
        }
    }

    fn capabilities(&mut self) -> Result<DeviceCapabilities, SeesawError<<D as I2cBus>::I2cError>> {
        let addr = self.addr();
        self.bus().lock(|driver| {
            driver
                .read_u32(addr, STATUS_OPTIONS)
                .map(|opts| opts.into())
                .map_err(SeesawError::I2c)
        })
    }

    fn hardware_id(&mut self) -> Result<u8, SeesawError<<D as I2cBus>::I2cError>> {
        let addr = self.addr();
        self.bus()
            .lock(|driver| driver.read_u8(addr, STATUS_HW_ID).map_err(SeesawError::I2c))
    }

    fn product_info(&mut self) -> Result<ProductDateCode, SeesawError<<D as I2cBus>::I2cError>> {
        let addr = self.addr();
        self.bus().lock(|driver| {
            driver
                .read_u32(addr, STATUS_VERSION)
                .map(|version| version.into())
                .map_err(SeesawError::I2c)
        })
    }

    fn reset(&mut self) -> Result<(), SeesawError<<D as I2cBus>::I2cError>> {
        let addr = self.addr();
        self.bus().lock(|driver| {
            driver
                .write_u8(addr, STATUS_SWRST, 0xFF)
                .map(|_| driver.delay_us(125_000))
                .map_err(SeesawError::I2c)
        })
    }

    fn temp(&mut self) -> Result<f32, SeesawError<<D as I2cBus>::I2cError>> {
        let addr = self.addr();

        self.bus().lock(|driver| {
            driver
                .read_u32(addr, STATUS_TEMP)
                .map(|buf| (buf as f32 / (1u32 << 16) as f32))
                .map_err(SeesawError::I2c)
        })
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
