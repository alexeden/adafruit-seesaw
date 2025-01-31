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

pub trait EncoderModule<D: Driver, const N: usize>: GpioModule<D> {
    const ENCODER_BTN_PINS: [u8; N];

    fn disable_button(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PINS[encoder], PinMode::OpenDrain)
            .map(|_| self.driver().delay_us(125))
    }
    fn enable_button(&mut self, encoder: usize) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PINS[encoder], PinMode::InputPullup)
            .map(|_| self.driver().delay_us(125))
    }

    fn button(&mut self, encoder: usize) -> Result<bool, SeesawError<D::Error>> {
        self.digital_read(Self::ENCODER_BTN_PINS[encoder])
    }

    fn delta(&mut self, encoder: u16) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = (u16::from_be_bytes(*DELTA) | encoder).to_be_bytes();
        self.driver().read_i32(addr, &reg).map_err(SeesawError::I2c)
    }

    fn disable_interrupt(&mut self, encoder: u16) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = (u16::from_be_bytes(*INT_CLR) | encoder).to_be_bytes();
        self.driver()
            .write_u8(addr, &reg, 1)
            .map_err(SeesawError::I2c)
    }

    fn enable_interrupt(&mut self, encoder: u16) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = (u16::from_be_bytes(*INT_SET) | encoder).to_be_bytes();
        self.driver()
            .write_u8(addr, &reg, 1)
            .map_err(SeesawError::I2c)
    }

    fn position(&mut self, encoder: u16) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = (u16::from_be_bytes(*POSITION) + encoder).to_be_bytes();
        self.driver().read_i32(addr, &reg).map_err(SeesawError::I2c)
    }

    fn set_position(&mut self, encoder: u16, pos: i32) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        let reg = (u16::from_be_bytes(*POSITION) + encoder).to_be_bytes();
        self.driver()
            .write_i32(addr, &reg, pos)
            .map_err(SeesawError::I2c)
    }
}
