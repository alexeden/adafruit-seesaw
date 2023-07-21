use crate::{
    common::{Modules, Reg},
    DriverExt, HardwareId,
};

/// RO - 8 bits
#[allow(dead_code)]
const STATUS: &Reg = &[Modules::Adc.into_u8(), 0x00];

/// WO - 8 bits
/// Writing a 1 to any bit in this register enables the corresponding interrupt.
/// Writing zeros to this register has no effect.
#[allow(dead_code)]
const INTENSET: &Reg = &[Modules::Adc.into_u8(), 0x02];

/// NOT SUPPORTED BY SEESAW PLATFORM
///
/// WO - 8 bits
/// Writing a 1 to any bit in this register enables the corresponding interrupt.
/// Writing zeros to this register has no effect.
#[allow(dead_code)]
const INTENCLR: &Reg = &[Modules::Adc.into_u8(), 0x03];

/// NOT SUPPORTED BY SEESAW PLATFORM
///
/// WO
/// Writing 1 to this register sets window control.
#[allow(dead_code)]
const WINMODE: &Reg = &[Modules::Adc.into_u8(), 0x04];

/// NOT SUPPORTED BY SEESAW PLATFORM
///
/// WO - 32 bits
/// This register sets the threshold values for window mode.
/// B31 - B16: High threshold
/// B15 - B0: Low threshold
#[allow(dead_code)]
const WINTHRESH: &Reg = &[Modules::Adc.into_u8(), 0x05];

/// RO - 16bits
/// ADC value for channel 0
const CHANNEL_0: &Reg = &[Modules::Adc.into_u8(), 0x07];

/// The ADC provides the ability to measure analog voltages at 10-bit
/// resolution. The SAMD09 seesaw has 4 ADC inputs, the Attiny8x7 has 11 ADC
/// inputs.
///
/// The module base register address for the ADC is 0x09
///
/// Conversions can be read by reading the corresponding CHANNEL register.
///
/// When reading ADC data, there should be at least a 500 uS delay between
/// writing the register number you would like to read from and attempting to
/// read the data.
///
/// Allow a delay of at least 1ms in between sequential ADC reads on different
/// channels.
pub trait AdcModule<D: crate::Driver>: crate::SeesawDevice<Driver = D> {
    fn analog_read(&mut self, pin: u8) -> Result<u16, crate::SeesawError<D::I2cError>> {
        let pin_offset = match Self::HARDWARE_ID {
            HardwareId::ATTINY817 => pin,
            HardwareId::SAMD09 => match pin {
                2 => 0,
                3 => 1,
                4 => 2,
                5 => 3,
                _ => 0,
            },
        };

        let addr = self.addr();
        self.driver()
            .read_u16(addr, &[CHANNEL_0[0], CHANNEL_0[1] + pin_offset])
            .map_err(crate::SeesawError::I2c)
    }
}
