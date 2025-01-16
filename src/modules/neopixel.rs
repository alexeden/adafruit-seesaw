use super::{Modules, Reg};
use crate::{devices::SeesawDevice, driver::Driver, DriverExt, SeesawError};

/// WO - 8 bits
/// This register sets the pin number (PORTA) that is used for the NeoPixel
/// output.
const SET_PIN: &Reg = &[Modules::Neopixel.into_u8(), 0x01];
/// WO - 8 bits
/// The protocol speed. (see `NeopixelSpeed`) Default is 800khz.
const SET_SPEED: &Reg = &[Modules::Neopixel.into_u8(), 0x02];
/// WO - 16 bits
/// The number of bytes currently used for the pixel array. This is
/// dependent on when the pixels you are using are RGB or RGBW.
const SET_LEN: &Reg = &[Modules::Neopixel.into_u8(), 0x03];
/// WO - 256 bits (32 bytes)
/// The data buffer. The first 2 bytes are the start address, and the data
/// to write follows. Data should be written in blocks of maximum size 30
/// bytes at a time.
const SET_BUF: &Reg = &[Modules::Neopixel.into_u8(), 0x04];
/// W0 - Zero bits
/// Sending the SHOW command will cause the output to update. There's no
/// arguments/data after the command.
const SHOW: &Reg = &[Modules::Neopixel.into_u8(), 0x05];

pub type ColorRGB = (u8, u8, u8);

pub trait NeopixelModule<D: Driver>: SeesawDevice<Driver = D> {
    const PIN: u8;

    /// The number of neopixels on the device
    const N_LEDS: u16 = 1;

    fn enable_neopixel(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();

        self.driver()
            .write_u8(addr, SET_PIN, Self::PIN)
            .and_then(|_| {
                self.driver().delay_us(10_000);
                self.driver().write_u16(addr, SET_LEN, 3 * Self::N_LEDS)
            })
            .map(|_| self.driver().delay_us(10_000))
            .map_err(SeesawError::I2c)
    }

    fn set_neopixel_speed(&mut self, speed: NeopixelSpeed) -> Result<(), SeesawError<D::Error>> {
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
            .map_err(SeesawError::I2c)
    }

    fn set_neopixel_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), SeesawError<D::Error>> {
        self.set_nth_neopixel_color(0, r, g, b)
    }

    fn set_nth_neopixel_color(
        &mut self,
        n: u16,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), SeesawError<D::Error>> {
        assert!(n < Self::N_LEDS);
        let [zero, one] = u16::to_be_bytes(3 * n);
        let addr = self.addr();

        self.driver()
            .register_write(addr, SET_BUF, &[zero, one, r, g, b])
            .map_err(SeesawError::I2c)
    }

    fn set_neopixel_colors<const N: usize>(
        &mut self,
        colors: &[ColorRGB; N],
    ) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();

        (0..N)
            .try_for_each(|n| {
                let [zero, one] = u16::to_be_bytes(3 * n as u16);
                let color = colors[n];
                self.driver()
                    .register_write(addr, SET_BUF, &[zero, one, color.0, color.1, color.2])
            })
            .map_err(SeesawError::I2c)
    }

    fn sync_neopixel(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();

        self.driver()
            .register_write(addr, SHOW, &[])
            .map(|_| self.driver().delay_us(125))
            .map_err(SeesawError::I2c)
    }
}

/// NeopixelModule: The Neopixel protocol speed
#[derive(Debug, Default)]
pub enum NeopixelSpeed {
    Khz400 = 0,
    #[default]
    Khz800 = 1,
}
