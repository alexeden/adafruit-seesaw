use super::{Modules, Reg};
use crate::DriverExt;

/// WO - 8 bits
/// Not documented.
pub const STATUS: &Reg = &[Modules::Neopixel.into(), 0x00];
/// WO - 8 bits
/// This register sets the pin number (PORTA) that is used for the NeoPixel
/// output.
pub const SET_PIN: &Reg = &[Modules::Neopixel.into(), 0x01];
/// WO - 8 bits
/// The protocol speed. (see `NeopixelSpeed`) Default is 800khz.
pub const SET_SPEED: &Reg = &[Modules::Neopixel.into(), 0x02];
/// WO - 16 bits
/// The number of bytes currently used for the pixel array. This is
/// dependent on when the pixels you are using are RGB or RGBW.
pub const SET_LEN: &Reg = &[Modules::Neopixel.into(), 0x03];
/// WO - 256 bits (32 bytes)
/// The data buffer. The first 2 bytes are the start address, and the data
/// to write follows. Data should be written in blocks of maximum size 30
/// bytes at a time.
pub const SET_BUF: &Reg = &[Modules::Neopixel.into(), 0x04];
/// W0 - Zero bits
/// Sending the SHOW command will cause the output to update. There's no
/// arguments/data after the command.
pub const SHOW: &Reg = &[Modules::Neopixel.into(), 0x05];

/// The Neopixel protocol speed
#[derive(Debug, Default)]
pub enum NeopixelSpeed {
    Khz400 = 0,
    #[default]
    Khz800 = 1,
}

pub trait NeopixelModule<D: crate::Driver>: crate::Device<D> {
    const PIN: u8;

    /// The number of neopixels on the device
    const N_LEDS: u16 = 1;

    fn enable_neopixel(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .write_u8(addr, SET_PIN, Self::PIN)
            .and_then(|_| {
                self.driver().delay_us(10_000);
                self.driver().write_u16(addr, SET_LEN, 3 * Self::N_LEDS)
            })
            .map(|_| self.driver().delay_us(10_000))
            .map_err(crate::SeesawError::I2c)
    }

    fn set_neopixel_speed(
        &mut self,
        speed: NeopixelSpeed,
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .write_u8(
                addr,
                SET_SPEED,
                match speed {
                    NeopixelSpeed::Khz400 => 0,
                    NeopixelSpeed::Khz800 => 1,
                },
            )
            .map(|_| self.driver().delay_us(10_000))
            .map_err(crate::SeesawError::I2c)
    }

    fn set_neopixel_color(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_nth_neopixel_color(0, r, g, b)
    }

    fn set_nth_neopixel_color(
        &mut self,
        n: u16,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        assert!(n < Self::N_LEDS as u16);
        let [zero, one] = u16::to_be_bytes(3 * n);
        let addr = self.addr();

        self.driver()
            .register_write(addr, SET_BUF, &[zero, one, r, g, b, 0x00])
            .map_err(crate::SeesawError::I2c)
    }

    fn set_neopixel_colors(
        &mut self,
        colors: &[(u8, u8, u8); Self::N_LEDS as usize],
    ) -> Result<(), crate::SeesawError<D::I2cError>>
    where
        [(); Self::N_LEDS as usize]: Sized,
    {
        let addr = self.addr();

        (0..Self::N_LEDS)
            .into_iter()
            .try_for_each(|n| {
                let [zero, one] = u16::to_be_bytes(3 * n);
                let color = colors[n as usize];
                self.driver().register_write(
                    addr,
                    SET_BUF,
                    &[zero, one, color.0, color.1, color.2, 0x00],
                )
            })
            .map_err(crate::SeesawError::I2c)
    }

    fn sync_neopixel(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .register_write(addr, SHOW, &[])
            .map(|_| self.driver().delay_us(125))
            .map_err(crate::SeesawError::I2c)
    }
}
