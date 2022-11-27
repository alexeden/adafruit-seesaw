// use super::{Addressable, SeesawDevice};
// use crate::{
//     bus::Attached,
//     error::SeesawError,
//     modules::{neopixel::NeopixelModule, status::StatusModule},
// };
// use embedded_hal::blocking::i2c::SevenBitAddress;

// pub struct NeoSlider<B>(SevenBitAddress, B);

// impl<B: crate::I2cBus> Addressable for NeoSlider<B> {
//     fn addr(&self) -> SevenBitAddress {
//         self.0
//     }
// }

// impl<B: crate::I2cBus> Attached<B> for NeoSlider<B> {
//     fn bus(&mut self) -> &mut B {
//         &mut self.1
//     }
// }

// impl<B: crate::I2cBus> NeopixelModule<B> for NeoSlider<B> {
//     const N_LEDS: u16 = 4;
//     const PIN: u8 = 14;
// }

// impl<B: crate::I2cBus> SeesawDevice<B> for NeoSlider<B> {
//     const DEFAULT_ADDR: u8 = 0x30;

//     fn begin(bus: B, addr: SevenBitAddress) -> Result<Self,
// SeesawError<B::I2cError>> {         let mut device = NeoSlider(addr, bus);
//         device.reset_and_begin()?;
//         device.enable_neopixel()?;
//         Ok(device)
//     }
// }
