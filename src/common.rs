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

pub const MODULE_STATUS: u8 = 0x00;
pub const MODULE_GPIO: u8 = 0x01;
pub const MODULE_SERCOM0: u8 = 0x02;
pub const MODULE_TIMER: u8 = 0x08;
pub const MODULE_ADC: u8 = 0x09;
/// `Dac` has a value in the C++ Seesaw library but is not used
pub const MODULE_DAC: u8 = 0x0A;
/// `Interrupt` has a value in the C++ Seesaw library but is not used
pub const MODULE_INTERRUPT: u8 = 0x0B;
/// `Dap` has a value in the C++ Seesaw library but is not used
pub const MODULE_DAP: u8 = 0x0C;
pub const MODULE_EEPROM: u8 = 0x0D;
pub const MODULE_NEOPIXEL: u8 = 0x0E;
pub const MODULE_TOUCH: u8 = 0x0F;
pub const MODULE_KEYPAD: u8 = 0x10;
pub const MODULE_ENCODER: u8 = 0x11;
pub const MODULE_SPECTRUM: u8 = 0x12;
