use super::{
    gpio::{GpioModule, PinMode},
    Reg, ENCODER_MODULE_ID,
};
use crate::{bus::Bus, error::SeesawError};

const STATUS: &Reg = &[ENCODER_MODULE_ID, 0x00];
const INT_SET: &Reg = &[ENCODER_MODULE_ID, 0x10];
const INT_CLR: &Reg = &[ENCODER_MODULE_ID, 0x20];
const POSITION: &Reg = &[ENCODER_MODULE_ID, 0x30];
const DELTA: &Reg = &[ENCODER_MODULE_ID, 0x40];

const ENCODER_BTN_PIN: u8 = 24;

pub trait EncoderModule<E, B: crate::Bus<E>>: GpioModule<E, B> {
    fn enable_button(&mut self) -> Result<(), SeesawError<E>> {
        self.set_pin_mode(ENCODER_BTN_PIN, PinMode::InputPullup)
            .map(|_| self.bus().delay_us(125))
    }

    fn button(&mut self) -> Result<bool, SeesawError<E>> {
        self.digital_read(ENCODER_BTN_PIN)
    }

    fn delta(&mut self) -> Result<i32, SeesawError<E>> {
        let addr = self.addr();
        self.bus().read_i32(addr, DELTA)
    }

    fn disable_interrupt(&mut self) -> Result<(), SeesawError<E>> {
        let addr = self.addr();
        self.bus().write_u8(addr, INT_CLR, 1)
    }

    fn enable_interrupt(&mut self) -> Result<(), SeesawError<E>> {
        let addr = self.addr();
        self.bus().write_u8(addr, INT_SET, 1)
    }

    fn position(&mut self) -> Result<i32, SeesawError<E>> {
        let addr = self.addr();
        self.bus().read_i32(addr, POSITION)
    }

    fn set_position(&mut self, pos: i32) -> Result<(), SeesawError<E>> {
        let addr = self.addr();
        self.bus().write_i32(addr, POSITION, pos)
    }
}
