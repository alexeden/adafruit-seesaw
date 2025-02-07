use super::{Modules, Reg};
use crate::{devices::SeesawDevice, Driver, DriverExt, HardwareId, SeesawError};

/// WO - 16 bits
/// The first byte of the register indicates which PWM pin will have its value
/// set The second byte is the actual PWM value
const PWM_VAL: &Reg = &[Modules::Timer.into_u8(), 0x01];

/// The PWM module provides up to 4 8-bit PWM outputs.
/// The module base register address for the PWM module is 0x08.
/// PWM outputs are available on pins PA04, PA05, PA06, and PA07.
pub trait TimerModule<D: Driver>: SeesawDevice<Driver = D> {
    /// Write a PWM value to a PWM-enabled pin
    ///
    /// On the SAMD09 breakout, the pin corresponds to the number on the
    /// silkscreen. On the default seesaw firmware on the SAMD09 breakout,
    /// pins 5, 6, and 7 are PWM enabled.
    fn analog_write(&mut self, pin: u8, value: u8) -> Result<(), SeesawError<D::Error>> {
        let mapped_pin = match Self::HARDWARE_ID {
            HardwareId::SAMD09 => match pin {
                4 => 0,
                5 => 1,
                6 => 2,
                7 => 3,
                _ => 0,
            },
            _ => pin,
        };

        let addr = self.addr();
        self.driver()
            .write_u16(addr, PWM_VAL, u16::from_be_bytes([mapped_pin, value]))
            .map_err(SeesawError::I2c)
    }
}
