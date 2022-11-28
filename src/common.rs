pub type Reg = [u8; 2];

pub const SEESAW_HW_ID: u8 = 0x55;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Modules {
    Status = 0x00,
    Gpio = 0x01,
    Sercom0 = 0x02,
    Timer = 0x08,
    Adc = 0x09,
    Dac = 0x0A,
    Interrupt = 0x0B,
    Dap = 0x0C,
    Eeprom = 0x0D,
    Neopixel = 0x0E,
    Touch = 0x0F,
    Keypad = 0x10,
    Encoder = 0x11,
    Spectrum = 0x12,
}

impl const From<Modules> for u8 {
    fn from(value: Modules) -> Self {
        value as u8
    }
}

/// NeopixelModule: The Neopixel protocol speed
#[derive(Debug, Default)]
pub enum NeopixelSpeed {
    Khz400 = 0,
    #[default]
    Khz800 = 1,
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
