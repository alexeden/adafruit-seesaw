pub mod adc;
pub mod encoder;
pub mod gpio;
pub mod keypad;
pub mod neopixel;
pub mod status;
pub mod timer;

pub type Reg = [u8; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareId {
    ATTINY817 = 0x87,
    SAMD09 = 0x55,
}

impl From<HardwareId> for u8 {
    fn from(value: HardwareId) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Modules {
    Status = 0x00,
    Gpio = 0x01,
    Sercom0 = 0x02,
    Timer = 0x08,
    Adc = 0x09,
    /// `Dac` has a value in the C++ Seesaw library but is not used
    Dac = 0x0A,
    /// `Interrupt` has a value in the C++ Seesaw library but is not used
    Interrupt = 0x0B,
    /// `Dap` has a value in the C++ Seesaw library but is not used
    Dap = 0x0C,
    Eeprom = 0x0D,
    Neopixel = 0x0E,
    Touch = 0x0F,
    Keypad = 0x10,
    Encoder = 0x11,
    Spectrum = 0x12,
}

impl Modules {
    pub const fn into_u8(self) -> u8 {
        self as u8
    }
}
