use crate::{
    common::{Modules, Reg},
    DriverExt, HardwareId,
};

/// WO - 16 bits
/// The first byte of the register indicates which PWM pin will have its value
/// set The second byte is the actual PWM value
const PWM_VAL: &Reg = &[Modules::Timer.into_u8(), 0x01];

/// The PWM module provides up to 4 8-bit PWM outputs.
/// The module base register address for the PWM module is 0x08.
/// PWM outputs are available on pins PA04, PA05, PA06, and PA07.
pub trait TimerModule<D: crate::Driver>: crate::SeesawDevice<Driver = D> {
    fn analog_write(&mut self, pin: u8, value: u8) -> Result<(), crate::SeesawError<D::I2cError>> {
        let mapped_pin = match Self::HARDWARE_ID {
            HardwareId::ATTINY817 => pin,
            HardwareId::SAMD09 => match pin {
                4 => 0,
                5 => 1,
                6 => 2,
                7 => 3,
                _ => 0,
            },
        };

        let addr = self.addr();
        self.driver()
            .write_u16(addr, PWM_VAL, u16::from_be_bytes([mapped_pin, value]))
            .map_err(crate::SeesawError::I2c)
    }
}
