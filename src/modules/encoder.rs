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

pub trait EncoderModule: GpioModule {
    fn enable_button<E, B: Bus<E>>(&mut self, bus: &mut B) -> Result<(), SeesawError<E>> {
        self.set_pin_mode(bus, ENCODER_BTN_PIN, PinMode::InputPullup)
            .map(|_| bus.delay_us(125))
    }

    fn button<E, B: Bus<E>>(&self, bus: &mut B) -> Result<bool, SeesawError<E>> {
        self.digital_read(bus, ENCODER_BTN_PIN)
    }

    fn delta<E, B: Bus<E>>(&self, bus: &mut B) -> Result<i32, SeesawError<E>> {
        bus.read_i32(self.addr(), DELTA)
    }

    fn disable_interrupt<E, B: Bus<E>>(&self, bus: &mut B) -> Result<(), SeesawError<E>> {
        bus.write_u8(self.addr(), INT_CLR, 1)
    }

    fn enable_interrupt<E, B: Bus<E>>(&self, bus: &mut B) -> Result<(), SeesawError<E>> {
        bus.write_u8(self.addr(), INT_SET, 1)
    }

    fn position<E, B: Bus<E>>(&self, bus: &mut B) -> Result<i32, SeesawError<E>> {
        bus.read_i32(self.addr(), POSITION)
    }

    fn set_position<E, B: Bus<E>>(&self, bus: &mut B, pos: i32) -> Result<(), SeesawError<E>> {
        bus.write_i32(self.addr(), POSITION, pos)
    }
}
