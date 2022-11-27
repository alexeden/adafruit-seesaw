// use super::{Addressable, SeesawDevice};
// use crate::{
//     bus::Attached,
//     error::SeesawError,
//     modules::{
//         gpio::{GpioModule, PinMode},
//         neopixel::NeopixelModule,
//         status::StatusModule,
//     },
// };
// use embedded_hal::blocking::i2c;

// const NEOKEY_1X4_PINMASK: u32 = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

// pub struct NeoKey1x4<B>(i2c::SevenBitAddress, B);

// impl<B> Addressable for NeoKey1x4<B> {
//     fn addr(&self) -> i2c::SevenBitAddress {
//         self.0
//     }
// }

// impl<B: crate::I2cBus> Attached<B> for NeoKey1x4<B> {
//     fn bus(&mut self) -> &mut B {
//         &mut self.1
//     }
// }

// impl<B: crate::I2cBus> GpioModule<B> for NeoKey1x4<B> {}
// impl<B: crate::I2cBus> NeopixelModule<B> for NeoKey1x4<B> {
//     const N_LEDS: u16 = 4;
//     const PIN: u8 = 3;
// }

// impl<B: crate::I2cBus> SeesawDevice<B> for NeoKey1x4<B> {
//     const DEFAULT_ADDR: u8 = 0x30;

//     fn begin(bus: B, addr: i2c::SevenBitAddress) -> Result<Self,
// SeesawError<B::I2cError>> {         let mut device = NeoKey1x4(addr, bus);
//         device
//             .reset_and_begin()
//             .and_then(|_| device.enable_neopixel())
//             .and_then(|_| device.set_pin_mode_bulk(NEOKEY_1X4_PINMASK,
// PinMode::InputPullup))             .map(|_| device)
//     }
// }

// // Additional methods
// impl<B: crate::I2cBus> NeoKey1x4<B>
// where
//     Self: GpioModule<B>,
// {
//     pub fn keys(&mut self) -> Result<u8, SeesawError<B::I2cError>> {
//         self.digital_read_bulk().map(|r| (r >> 4 & 0xF) as u8)
//     }

//     pub fn keys_bool(&mut self) -> Result<[bool; 4],
// SeesawError<B::I2cError>> {         self.keys().map(|b| {
//             [
//                 0 == 1 & b >> 0,
//                 0 == 1 & b >> 1,
//                 0 == 1 & b >> 2,
//                 0 == 1 & b >> 3,
//             ]
//         })
//     }
// }
