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

pub trait EncoderModule<D: Driver, const N_ENCODERS: usize>: GpioModule<D> {
    const ENCODER_BTN_PINS: [u8; N_ENCODERS];

    fn enable_button(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PINS[encoder], PinMode::InputPullup)
            .map(|_| self.driver().delay_us(125))
    }

    fn button(&mut self, encoder: usize) -> Result<bool, SeesawError<D::Error>> {
        self.digital_read(Self::ENCODER_BTN_PINS[encoder])
    }

    fn delta(&mut self, encoder: usize) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = &[DELTA[0], DELTA[1] | encoder as u8];
        self.driver().read_i32(addr, reg).map_err(SeesawError::I2c)
    }

    fn disable_interrupt(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = &[INT_CLR[0], INT_CLR[1] | encoder as u8];
        self.driver()
            .write_u8(addr, reg, 1)
            .map_err(SeesawError::I2c)
    }

    fn enable_interrupt(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = &[INT_SET[0], INT_SET[1] | encoder as u8];
        self.driver()
            .write_u8(addr, reg, 1)
            .map_err(SeesawError::I2c)
    }

    fn position(&mut self, encoder: usize) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = &[POSITION[0], POSITION[1] | encoder as u8];
        self.driver().read_i32(addr, reg).map_err(SeesawError::I2c)
    }

    fn set_position(&mut self, encoder: usize, pos: i32) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = &[POSITION[0], POSITION[1] | encoder as u8];
        self.driver()
            .write_i32(addr, reg, pos)
            .map_err(SeesawError::I2c)
    }
}
