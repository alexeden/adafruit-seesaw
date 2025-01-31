use super::{
    gpio::{GpioModule, PinMode},
    Modules, Reg,
};
use crate::{Driver, DriverExt, SeesawError};

#[allow(dead_code)]
const STATUS: &Reg = &[Modules::Encoder.into_u8(), 0x00];
const INT_SET: &Reg = &[Modules::Encoder.into_u8(), 0x10];
const INT_CLR: &Reg = &[Modules::Encoder.into_u8(), 0x20];
const POSITION: &Reg = &[Modules::Encoder.into_u8(), 0x30];
const DELTA: &Reg = &[Modules::Encoder.into_u8(), 0x40];

pub trait QuadEncoderModule<D: Driver>: GpioModule<D> {
    const ENCODER_ID: [u8; 4] = [0x0, 0x1, 0x2, 0x3];
    const ENCODER_BTN_PIN: [u8; 4] = [12, 14, 17, 9];

    fn enable_button(&mut self, button: usize) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PIN[button], PinMode::InputPullup)
            .map(|_| self.driver().delay_us(125))
    }

    fn button(&mut self, button: usize) -> Result<bool, SeesawError<D::Error>> {
        self.digital_read(Self::ENCODER_BTN_PIN[button])
    }

    fn delta(&mut self, encoder: usize) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, &[DELTA[0], DELTA[1] | Self::ENCODER_ID[encoder]])
            .map_err(SeesawError::I2c)
    }

    fn disable_interrupt(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(
                addr,
                &[INT_CLR[0], INT_CLR[1] | Self::ENCODER_ID[encoder]],
                1,
            )
            .map_err(SeesawError::I2c)
    }

    fn enable_interrupt(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(
                addr,
                &[INT_SET[0], INT_SET[1] | Self::ENCODER_ID[encoder]],
                1,
            )
            .map_err(SeesawError::I2c)
    }

    fn position(&mut self, encoder: usize) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(
                addr,
                &[POSITION[0], POSITION[1] | Self::ENCODER_ID[encoder]],
            )
            .map_err(SeesawError::I2c)
    }

    fn set_position(&mut self, encoder: usize, pos: i32) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_i32(
                addr,
                &[POSITION[0], POSITION[1] | Self::ENCODER_ID[encoder]],
                pos,
            )
            .map_err(SeesawError::I2c)
    }
}
