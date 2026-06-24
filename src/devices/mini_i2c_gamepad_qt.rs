use super::SeesawDeviceInit;
use crate::{
    modules::{
        adc::AdcModule,
        gpio::{GpioModule, PinMode},
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

// Source: https://github.com/adafruit/Adafruit_Seesaw/blob/master/examples/Mini_I2C_Gamepad_QT/Mini_I2C_Gamepad_QT.ino
const BUTTON_X: u8 = 6;
const BUTTON_Y: u8 = 2;
const BUTTON_A: u8 = 5;
const BUTTON_B: u8 = 1;
const BUTTON_SELECT: u8 = 0;
const BUTTON_START: u8 = 16;
const BUTTON_MASK: u32 = (1 << BUTTON_X)
    | (1 << BUTTON_Y)
    | (1 << BUTTON_A)
    | (1 << BUTTON_B)
    | (1 << BUTTON_SELECT)
    | (1 << BUTTON_START);

const JOYSTICK_X: u8 = 14;
const JOYSTICK_Y: u8 = 15;
const JOYSTICK_MAX: u16 = 1023;

seesaw_device! {
  /// MiniI2cGamepadQt
  ///
  /// Button | Pin
  /// ---|---
  /// X | 6
  /// Y | 2
  /// A | 5
  /// B | 1
  /// SELECT | 0
  /// START | 16
  ///
  /// Joystick | ADC Pin
  /// ---|---
  /// X | 14
  /// Y | 15
  ///
  name: MiniI2cGamepadQt,
  hardware_id: HardwareId::ATTINY817,
  product_id: 5743,
  default_addr: 0x50
}

impl<D: Driver> AdcModule<D> for MiniI2cGamepadQt<D> {}
impl<D: Driver> GpioModule<D> for MiniI2cGamepadQt<D> {}

impl<D: Driver> SeesawDeviceInit<D> for MiniI2cGamepadQt<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        self.enable_buttons()?;
        Ok(self)
    }
}

impl<D: Driver> MiniI2cGamepadQt<D> {
    pub fn state(&mut self) -> Result<MiniI2cGamepadQtState, SeesawError<D::Error>> {
        let x = JOYSTICK_MAX.saturating_sub(self.analog_read(JOYSTICK_X)?);
        let y = JOYSTICK_MAX.saturating_sub(self.analog_read(JOYSTICK_Y)?);
        let buttons = MiniI2cGamepadQtButtons::from_bits(self.digital_read_bulk()?);

        Ok(MiniI2cGamepadQtState { x, y, buttons })
    }

    pub fn enable_buttons(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode_bulk(BUTTON_MASK, PinMode::InputPullup)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MiniI2cGamepadQtState {
    pub x: u16,
    pub y: u16,
    pub buttons: MiniI2cGamepadQtButtons,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MiniI2cGamepadQtButtons {
    pub x: bool,
    pub y: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl MiniI2cGamepadQtButtons {
    fn from_bits(bits: u32) -> Self {
        Self {
            x: pressed(bits, BUTTON_X),
            y: pressed(bits, BUTTON_Y),
            a: pressed(bits, BUTTON_A),
            b: pressed(bits, BUTTON_B),
            select: pressed(bits, BUTTON_SELECT),
            start: pressed(bits, BUTTON_START),
        }
    }
}

fn pressed(bits: u32, pin: u8) -> bool {
    bits & (1 << pin) == 0
}
