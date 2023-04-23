use super::gpio::{GpioModule, PinMode};
use crate::{
    common::Reg,
    DriverExt, MODULE_ENCODER,
};

#[allow(dead_code)]
const STATUS: &Reg = &[MODULE_ENCODER, 0x00];
const INT_SET: &Reg = &[MODULE_ENCODER, 0x10];
const INT_CLR: &Reg = &[MODULE_ENCODER, 0x20];
const POSITION: &Reg = &[MODULE_ENCODER, 0x30];
const DELTA: &Reg = &[MODULE_ENCODER, 0x40];

pub trait EncoderModule<D: crate::Driver>: GpioModule<D> {
    const ENCODER_BTN_PIN: u8;

    fn enable_button(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_pin_mode(Self::ENCODER_BTN_PIN, PinMode::InputPullup)
            .map(|_| self.driver().delay_us(125))
    }

    fn button(&mut self) -> Result<bool, crate::SeesawError<D::I2cError>> {
        self.digital_read(Self::ENCODER_BTN_PIN)
    }

    fn delta(&mut self) -> Result<i32, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, DELTA)
            .map_err(crate::SeesawError::I2c)
    }

    fn disable_interrupt(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_CLR, 1)
            .map_err(crate::SeesawError::I2c)
    }

    fn enable_interrupt(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_SET, 1)
            .map_err(crate::SeesawError::I2c)
    }

    fn position(&mut self) -> Result<i32, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_i32(addr, POSITION)
            .map_err(crate::SeesawError::I2c)
    }

    fn set_position(&mut self, pos: i32) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .write_i32(addr, POSITION, pos)
            .map_err(crate::SeesawError::I2c)
    }
}
