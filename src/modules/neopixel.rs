use super::{Reg, NEOPIXEL_MODULE_ID};
use crate::{bus::Attached, devices::Addressable, error::SeesawError};

/// WO - 8 bits
/// Not documented.
pub const STATUS: &Reg = &[NEOPIXEL_MODULE_ID, 0x00];
/// WO - 8 bits
/// This register sets the pin number (PORTA) that is used for the NeoPixel
/// output.
pub const SET_PIN: &Reg = &[NEOPIXEL_MODULE_ID, 0x01];
/// WO - 8 bits
/// The protocol speed. (see `NeopixelSpeed`) Default is 800khz.
pub const SET_SPEED: &Reg = &[NEOPIXEL_MODULE_ID, 0x02];
/// WO - 16 bits
/// The number of bytes currently used for the pixel array. This is
/// dependent on when the pixels you are using are RGB or RGBW.
pub const SET_LEN: &Reg = &[NEOPIXEL_MODULE_ID, 0x03];
/// WO - 256 bits (32 bytes)
/// The data buffer. The first 2 bytes are the start address, and the data
/// to write follows. Data should be written in blocks of maximum size 30
/// bytes at a time.
pub const SET_BUF: &Reg = &[NEOPIXEL_MODULE_ID, 0x04];
/// W0 - Zero bits
/// Sending the SHOW command will cause the output to update. There's no
/// arguments/data after the command.
pub const SHOW: &Reg = &[NEOPIXEL_MODULE_ID, 0x05];

/// The Neopixel protocol speed
#[derive(Debug, Default)]
pub enum NeopixelSpeed {
    Khz400 = 0,
    #[default]
    Khz800 = 1,
}

pub trait NeopixelModule<E, B: crate::Bus<E>>: Addressable + Attached<E, B> {
    const PIN: u8;

    /// The number of neopixels on the device
    const N_LEDS: u16 = 1;

    fn enable_neopixel(&mut self) -> Result<(), SeesawError<E>> {
        self.bus()
            .write_u8(self.addr(), SET_PIN, Self::PIN)
            .and_then(|_| {
                self.bus().delay_us(10_000);
                self.bus().write_u16(self.addr(), SET_LEN, 3 * Self::N_LEDS)
            })
            .map(|_| self.bus().delay_us(10_000))
    }

    fn set_neopixel_speed(&self, speed: NeopixelSpeed) -> Result<(), SeesawError<E>> {
        self.bus()
            .write_u8(
                self.addr(),
                SET_SPEED,
                match speed {
                    NeopixelSpeed::Khz400 => 0,
                    NeopixelSpeed::Khz800 => 1,
                },
            )
            .map(|_| self.bus().delay_us(10_000))
    }

    fn set_neopixel_color(&self, r: u8, g: u8, b: u8) -> Result<(), SeesawError<E>> {
        self.set_nth_neopixel_color(0, r, g, b)
    }

    fn set_nth_neopixel_color(&self, n: u16, r: u8, g: u8, b: u8) -> Result<(), SeesawError<E>> {
        assert!(n < Self::N_LEDS as u16);
        let [zero, one] = u16::to_be_bytes(3 * n);
        self.bus()
            .register_write(self.addr(), SET_BUF, &[zero, one, r, g, b, 0x00])
    }

    fn set_neopixel_colors(
        &self,
        colors: &[(u8, u8, u8); Self::N_LEDS as usize],
    ) -> Result<(), SeesawError<E>>
    where
        [(); Self::N_LEDS as usize]: Sized,
    {
        // assert!(n < Self::N_LEDS);
        (0..Self::N_LEDS).into_iter().try_for_each(|n| {
            let [zero, one] = u16::to_be_bytes(3 * n);
            let color = colors[n as usize];
            self.bus().register_write(
                self.addr(),
                SET_BUF,
                &[zero, one, color.0, color.1, color.2, 0x00],
            )
        })
    }

    fn sync_neopixel(&self) -> Result<(), SeesawError<E>> {
        self.bus()
            .register_write(self.addr(), SHOW, &[])
            .map(|_| self.bus().delay_us(125))
    }
}
