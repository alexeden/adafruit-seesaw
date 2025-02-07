use super::SeesawDeviceInit;
use crate::{
    modules::{
        gpio::{GpioModule, PinMode},
        status::StatusModule,
        timer::TimerModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
  /// ArcadeButton1x4
  ///
  /// Button | Pin | GPIO
  /// ---|---|---
  /// SW1 | 18 | PA01
  /// SW2 | 19 | PA02
  /// SW3 | 20 | PA03
  /// SW4 | 2 | PA06
  ///
  /// LED | PIN | GPIO
  /// ---|---|---
  /// PWM1 | 12 | PC00
  /// PWM2 | 13 | PC01
  /// PWM3 | 0 | PA04
  /// PWM4 | 1 | PA05
  ///
  name: ArcadeButton1x4,
  hardware_id: HardwareId::ATTINY817,
  product_id: 5296,
  default_addr: 0x3A
}

impl<D: Driver> GpioModule<D> for ArcadeButton1x4<D> {}
impl<D: Driver> TimerModule<D> for ArcadeButton1x4<D> {}

impl<D: Driver> SeesawDeviceInit<D> for ArcadeButton1x4<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_buttons())
            .map(|_| self)
    }
}

impl<D: Driver> ArcadeButton1x4<D> {
    pub fn button_values(&mut self) -> Result<[bool; 4], SeesawError<D::Error>> {
        [18, 19, 20, 2].try_map(|pin| self.digital_read(pin))
    }

    /// Set the pin mode of the 4 buttons to input pullup:
    pub fn enable_buttons(&mut self) -> Result<(), SeesawError<D::Error>> {
        self.set_pin_mode(18, PinMode::InputPullup)?;
        self.set_pin_mode(19, PinMode::InputPullup)?;
        self.set_pin_mode(20, PinMode::InputPullup)?;
        self.set_pin_mode(2, PinMode::InputPullup)?;
        Ok(())
    }

    pub fn set_led_duty_cycles(&mut self, pwms: &[u8; 4]) -> Result<(), SeesawError<D::Error>> {
        [12u8, 13, 0, 1]
            .iter()
            .enumerate()
            .try_for_each(|(i, &pin)| self.analog_write(pin, pwms[i]))
    }
}
