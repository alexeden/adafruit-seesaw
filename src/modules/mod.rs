#[cfg(feature = "module_adc")]
pub mod adc;
#[cfg(feature = "module_encoder")]
pub mod encoder;
#[cfg(feature = "module_gpio")]
pub mod gpio;
#[cfg(feature = "module_keypad")]
pub mod keypad;
#[cfg(feature = "module_neopixel")]
pub mod neopixel;
pub mod status;
#[cfg(feature = "module_timer")]
pub mod timer;
pub mod touch;

pub type Reg = [u8; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HardwareId {
    /// seesaw HW ID code for SAMD09
    SAMD09 = 0x55,
    /// seesaw HW ID code for ATtiny806
    ATTINY806 = 0x84,
    /// seesaw HW ID code for ATtiny807
    ATTINY807 = 0x85,
    /// seesaw HW ID code for ATtiny816
    ATTINY816 = 0x86,
    /// seesaw HW ID code for ATtiny817
    ATTINY817 = 0x87,
    /// seesaw HW ID code for ATtiny1616
    ATTINY1616 = 0x88,
    /// seesaw HW ID code for ATtiny1617
    ATTINY1617 = 0x89,
}

impl From<HardwareId> for u8 {
    fn from(value: HardwareId) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
