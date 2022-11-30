use crate::{
    common::{Modules, Reg},
    DriverExt,
};

/// WO - 32 bits
/// Writing a 1 to any bit in this register sets the direction of the
/// corresponding pin to OUTPUT. Writing 0 has no effect.
#[allow(dead_code)]
const SET_OUTPUT: &Reg = &[Modules::Gpio.into(), 0x02];

/// WO - 32 bits
/// Writing a 1 to any bit in this register sets the direction of the
/// corresponding pin to INPUT. Writing 0 has no effect.
const SET_INPUT: &Reg = &[Modules::Gpio.into(), 0x03];

/// WR - 32 bits
/// When written to, all bits that are set to 0 will have their
/// corresponding pins set LOW. All bits that are set to 1 will
/// have their corresponding pins set HIGH.
/// Reading this register reads all pins on PORTA of the seesaw device.
const GPIO: &Reg = &[Modules::Gpio.into(), 0x04];

/// WO - 32 bits
/// Writing a 1 to any bit in this register writes the corresponding pin
/// HIGH. Writing 0 has no effect.
const SET_HIGH: &Reg = &[Modules::Gpio.into(), 0x05];

/// WO - 32 bits
/// Writing a 1 to any bit in this register writes the corresponding pin
/// LOW. Writing 0 has no effect.
const SET_LOW: &Reg = &[Modules::Gpio.into(), 0x06];

/// W0 - 32 bits
/// Writing a 1 to any bit in this register toggles the corresponding pin.
/// Writing 0 has no effect.
#[allow(dead_code)]
const TOGGLE: &Reg = &[Modules::Gpio.into(), 0x07];

/// WO - 32 bits
/// Writing a 1 to any bit in this register enables the interrupt on the
/// corresponding pin. When the value on this pin changes, the corresponding
/// bit will be set in the INTFLAG register. Writing 0 has no effect.
#[allow(dead_code)]
const INT_ENABLE: &Reg = &[Modules::Gpio.into(), 0x08];

/// WO - 32 bits
/// Writing a 1 to any bit in this register disables the interrupt on the
/// corresponding pin. Writing 0 has no effect.
#[allow(dead_code)]
const INT_DISABLE: &Reg = &[Modules::Gpio.into(), 0x09];

/// RO - 32 bits
/// This register hold the status of all GPIO interrupts.
/// When an interrupt fires, the corresponding bit in this register gets
/// set. Reading this register clears all interrupts.
#[allow(dead_code)]
const INT_FLAG: &Reg = &[Modules::Gpio.into(), 0x0A];

/// WO - 32 bits
/// Writing a 1 to any bit in this register enables the internal pullup or
/// pulldown on the corresponding pin. The pull direction (up/down) is
/// determined by the GPIO (output) value - if the corresponding GPIO
/// register bit is low,  its a pulldown. High, its a pullup. Writing 0 has
/// no effect.
const PULL_ENABLE: &Reg = &[Modules::Gpio.into(), 0x0B];

/// WO - 32 bits
/// Writing a 1 to any bit in this register disables the pull up/down on the
/// corresponding pin. Writing 0 has no effect.
#[allow(dead_code)]
const PULL_DISABLE: &Reg = &[Modules::Gpio.into(), 0x0C];

pub trait GpioModule<D: crate::Driver>: crate::SeesawDevice<D> {
    fn digital_read(&mut self, pin: u8) -> Result<bool, crate::SeesawError<D::I2cError>> {
        self.digital_read_bulk()
            .map(|pins| !matches!(pins >> pin & 0x1, 1))
    }

    fn digital_read_bulk(&mut self) -> Result<u32, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_u32(addr, GPIO)
            .map_err(crate::SeesawError::I2c)
    }

    fn set_pin_mode(
        &mut self,
        pin: u8,
        mode: PinMode,
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_pin_mode_bulk(1 << pin, mode)
    }

    fn set_pin_mode_bulk(
        &mut self,
        pins: u32,
        mode: PinMode,
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        let bus = self.driver();

        match mode {
            PinMode::Output => bus.write_u32(addr, GPIO, pins),
            PinMode::Input => bus.write_u32(addr, SET_INPUT, pins),
            PinMode::InputPullup => bus
                .write_u32(addr, SET_INPUT, pins)
                .and_then(|_| bus.write_u32(addr, PULL_ENABLE, pins))
                .and_then(|_| bus.write_u32(addr, SET_HIGH, pins)),
            PinMode::InputPulldown => bus
                .write_u32(addr, SET_INPUT, pins)
                .and_then(|_| bus.write_u32(addr, PULL_ENABLE, pins))
                .and_then(|_| bus.write_u32(addr, SET_LOW, pins)),
            _ => unimplemented!("Other pins modes are not supported."),
        }
        .map_err(crate::SeesawError::I2c)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PinMode {
    Input = 0x01,
    Output = 0x02,
    Pullup = 0x04,
    InputPullup = 0x05,
    Pulldown = 0x08,
    InputPulldown = 0x09,
    OpenDrain = 0x10,
    OutputOpenDrain = 0x12,
    Special = 0xF0,
    Function1 = 0x00,
    Function2 = 0x20,
    Function3 = 0x40,
    Function4 = 0x60,
    Function5 = 0x80,
    Function6 = 0xA0,
    Analog = 0xC0,
}

impl const From<PinMode> for u8 {
    fn from(value: PinMode) -> Self {
        value as u8
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InterruptMode {
    Disabled = 0x00,
    Rising = 0x01,
    Falling = 0x02,
    Change = 0x03,
    Onlow = 0x04,
    Onhigh = 0x05,
    OnlowWe = 0x0C,
    OnhighWe = 0x0D,
}

impl const From<InterruptMode> for u8 {
    fn from(value: InterruptMode) -> Self {
        value as u8
    }
}
