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

pub trait EncoderModule<D: Driver>: GpioModule<D> {
    const ENCODER_BTN_PIN: u8;

    fn enable_button(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(Self::ENCODER_BTN_PIN, PinMode::InputPullup)
            .map(|_| self.driver().delay_us(125))
    }

    fn button(&mut self) -> Result<bool, SeesawError<D::Error>> {
        self.digital_read(Self::ENCODER_BTN_PIN)
    }

    fn delta(&mut self) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, DELTA)
            .map_err(SeesawError::I2c)
    }

    fn disable_interrupt(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_CLR, 1)
            .map_err(SeesawError::I2c)
    }

    fn enable_interrupt(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_SET, 1)
            .map_err(SeesawError::I2c)
    }

    fn position(&mut self) -> Result<i32, SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, POSITION)
            .map_err(SeesawError::I2c)
    }

    fn set_position(&mut self, pos: i32) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_i32(addr, POSITION, pos)
            .map_err(SeesawError::I2c)
    }
}
