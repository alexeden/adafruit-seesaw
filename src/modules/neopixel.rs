use super::{Modules, Reg};
use crate::{devices::SeesawDevice, driver::Driver, DriverExt, SeesawError};
use rgb::ComponentSlice;

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

pub trait NeopixelModule<D: Driver>: SeesawDevice<Driver = D> {
    /// The size of the color type in bytes
    const C_SIZE: usize = {
        match core::mem::size_of::<Self::Color>() {
            3 => 3,
            4 => 4,
            _ => panic!("Invalid color size"),
        }
    };
    /// The number of neopixels on or connected to the device
    const N_LEDS: usize = 1;
    /// The output pin of the neopixel signal
    const PIN: u8;

    type Color: ComponentSlice<u8>;

    /// Set which pin the device sends the neopixel signal through and
    /// set the length of its internal pixel buffer
    fn enable_neopixel(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();

        self.driver()
            .write_u8(addr, SET_PIN, Self::PIN)
            .map(|_| self.driver().delay_us(10_000))
            .and_then(|_| {
                self.driver()
                    .write_u16(addr, SET_LEN, (Self::C_SIZE * Self::N_LEDS) as u16)
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

    /// Set the color of the first (and, in the case of some devices, only)
    /// neopixel
    fn set_neopixel_color(&mut self, color: Self::Color) -> Result<(), SeesawError<D::Error>>
    where
        [(); 2 + Self::C_SIZE]: Sized,
    {
        self.set_nth_neopixel_color(0, color)
    }

    /// Set the color of the nth neopixel
    fn set_nth_neopixel_color(
        &mut self,
        n: usize,
        color: Self::Color,
    ) -> Result<(), SeesawError<D::Error>>
    where
        [(); 2 + Self::C_SIZE]: Sized,
    {
        assert!(n < Self::N_LEDS);
        let addr = self.addr();
        let mut buf = [0; 2 + Self::C_SIZE];
        buf[..2].copy_from_slice(&u16::to_be_bytes((Self::C_SIZE * n) as u16));
        buf[2..].copy_from_slice(color.as_slice());
        self.driver()
            .register_write(addr, SET_BUF, &buf)
            .map_err(SeesawError::I2c)
    }

    /// Set the color of all neopixels
    ///
    /// Minimizes the number of transactions performed by chunking the `colors`
    /// array into the largest (max 30 byte) buffers possible
    ///
    /// Note that if `C_SIZE` is _not_ 3 or 4 bytes, the chunking optimization
    /// is effectively skipped
    fn set_neopixel_colors(
        &mut self,
        colors: &[Self::Color; Self::N_LEDS],
    ) -> Result<(), SeesawError<D::Error>>
    where
        [(); 2 + color_bytes_per_write(Self::C_SIZE)]: Sized,
    {
        let mut buf = [0; 2 + color_bytes_per_write(Self::C_SIZE)];
        let addr = self.addr();

        colors
            .chunks(max_colors_per_write(Self::C_SIZE))
            .enumerate()
            .try_for_each(|(i, chunk)| {
                let offset = u16::to_be_bytes(
                    (Self::C_SIZE * i * max_colors_per_write(Self::C_SIZE)) as u16,
                );
                buf[..2].copy_from_slice(&offset);
                chunk.iter().enumerate().for_each(|(j, c)| {
                    let start = 2 + (j * Self::C_SIZE);
                    buf[start..start + Self::C_SIZE].copy_from_slice(c.as_slice());
                });

                self.driver().register_write(
                    addr,
                    SET_BUF,
                    &buf[0..2 + (Self::C_SIZE * chunk.len())],
                )
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

/// Get the maximum number of colors that can be written in a single write
/// operation as a function of the number of bytes per color
pub const fn max_colors_per_write(c_size: usize) -> usize {
    match c_size {
        3 => 9, // 27
        4 => 7, // 28
        _ => 1, // effectively skips the optimization
    }
}

/// Get the number of bytes dedicated to writing colors in a single write
/// operation as a function of the number of bytes per color
pub const fn color_bytes_per_write(c_size: usize) -> usize {
    c_size * max_colors_per_write(c_size)
}

/// NeopixelModule: The Neopixel protocol speed
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NeopixelSpeed {
    Khz400 = 0,
    #[default]
    Khz800 = 1,
}
