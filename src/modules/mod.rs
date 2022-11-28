// // pub mod encoder;
// // pub mod gpio;
// // pub mod neopixel;
// mod status;
// pub use status::*;

// pub type Reg = [u8; 2];

// pub const SEESAW_HW_ID: u8 = 0x55;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Modules {
//     Status = 0x00,
//     Gpio = 0x01,
//     Sercom0 = 0x02,
//     Timer = 0x08,
//     Adc = 0x09,
//     Dac = 0x0A,
//     Interrupt = 0x0B,
//     Dap = 0x0C,
//     Eeprom = 0x0D,
//     Neopixel = 0x0E,
//     Touch = 0x0F,
//     Keypad = 0x10,
//     Encoder = 0x11,
//     Spectrum = 0x12,
// }

// impl const From<Modules> for u8 {
//     fn from(value: Modules) -> Self {
//         value as u8
//     }
// }
