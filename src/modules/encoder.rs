// use super::{
//     gpio::{GpioModule, PinMode},
//     Reg, ENCODER_MODULE_ID,
// };
// use crate::{bus::I2cExt, error::SeesawError};

// const STATUS: &Reg = &[ENCODER_MODULE_ID, 0x00];
// const INT_SET: &Reg = &[ENCODER_MODULE_ID, 0x10];
// const INT_CLR: &Reg = &[ENCODER_MODULE_ID, 0x20];
// const POSITION: &Reg = &[ENCODER_MODULE_ID, 0x30];
// const DELTA: &Reg = &[ENCODER_MODULE_ID, 0x40];

// const ENCODER_BTN_PIN: u8 = 24;

// pub trait EncoderModule<B: crate::I2cBus>: GpioModule<B> {
//     fn enable_button(&mut self) -> Result<(), SeesawError<B::I2cError>> {
//         self.set_pin_mode(ENCODER_BTN_PIN, PinMode::InputPullup)
//             .map(|_| self.bus().delay_us(125))
//     }

//     fn button(&mut self) -> Result<bool, SeesawError<B::I2cError>> {
//         self.digital_read(ENCODER_BTN_PIN)
//     }

//     fn delta(&mut self) -> Result<i32, SeesawError<B::I2cError>> {
//         let addr = self.addr();
//         self.bus().read_i32(addr, DELTA).map_err(SeesawError::I2c)
//     }

//     fn disable_interrupt(&mut self) -> Result<(), SeesawError<B::I2cError>> {
//         let addr = self.addr();
//         self.bus()
//             .write_u8(addr, INT_CLR, 1)
//             .map_err(SeesawError::I2c)
//     }

//     fn enable_interrupt(&mut self) -> Result<(), SeesawError<B::I2cError>> {
//         let addr = self.addr();
//         self.bus()
//             .write_u8(addr, INT_SET, 1)
//             .map_err(SeesawError::I2c)
//     }

//     fn position(&mut self) -> Result<i32, SeesawError<B::I2cError>> {
//         let addr = self.addr();
//         self.bus()
//             .read_i32(addr, POSITION)
//             .map_err(SeesawError::I2c)
//     }

//     fn set_position(&mut self, pos: i32) -> Result<(),
// SeesawError<B::I2cError>> {         let addr = self.addr();
//         self.bus()
//             .write_i32(addr, POSITION, pos)
//             .map_err(SeesawError::I2c)
//     }
// }
