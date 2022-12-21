use crate::{
    driver::Driver,
    modules::{
        adc::AdcModule,
        encoder::EncoderModule,
        gpio::{GpioModule, PinMode},
        neopixel::NeopixelModule,
        status::StatusModule,
        timer::TimerModule,
    },
    seesaw_device, HardwareId, SeesawDeviceInit,
};

/// All devices implement the status module
impl<D: Driver, T: super::SeesawDevice<Driver = D>> StatusModule<D> for T {}

seesaw_device! {
    #[doc(hidden)]
    name: GenericDevice,
    hardware_id: HardwareId::SAMD09,
    product_id: 0,
    default_addr: 0x49,
    modules: []
}

impl<D: Driver> SeesawDeviceInit<D> for GenericDevice<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}

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
    default_addr: 0x3A,
    modules: [
        GpioModule,
        TimerModule
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for ArcadeButton1x4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_buttons())
            .map(|_| self)
    }
}

impl<D: Driver> ArcadeButton1x4<D> {
    pub fn button_values(&mut self) -> Result<[bool; 4], crate::SeesawError<D::I2cError>> {
        [18, 19, 20, 2].try_map(|pin| self.digital_read(pin))
    }

    /// Set the pin mode of the 4 buttons to input pullup:
    pub fn enable_buttons(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_pin_mode(18, PinMode::InputPullup)?;
        self.set_pin_mode(19, PinMode::InputPullup)?;
        self.set_pin_mode(20, PinMode::InputPullup)?;
        self.set_pin_mode(2, PinMode::InputPullup)?;
        Ok(())
    }

    pub fn set_led_duty_cycles(
        &mut self,
        pwms: &[u8; 4],
    ) -> Result<(), crate::SeesawError<D::I2cError>> {
        [12u8, 13, 0, 1]
            .iter()
            .enumerate()
            .try_for_each(|(i, &pin)| self.analog_write(pin, pwms[i]))
    }
}

seesaw_device! {
    /// NeoKey1x4
    name: NeoKey1x4,
    hardware_id: HardwareId::SAMD09,
    product_id: 4980,
    default_addr: 0x30,
    modules: [
        GpioModule,
        NeopixelModule { num_leds: 4, pin: 3 },
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoKey1x4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button_pins())
            .map(|_| self)
    }
}

impl<D: Driver> NeoKey1x4<D> {
    pub fn enable_button_pins(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        self.set_pin_mode_bulk(
            (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7),
            PinMode::InputPullup,
        )
    }

    pub fn keys(&mut self) -> Result<u8, crate::SeesawError<D::I2cError>> {
        self.digital_read_bulk().map(|r| (r >> 4 & 0xF) as u8)
    }
}

seesaw_device!(
    /// NeoSlider
    name: NeoSlider,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5295,
    default_addr: 0x30,
    modules: [
        AdcModule,
        GpioModule,
        NeopixelModule { num_leds: 4, pin: 14},
    ]
);

impl<D: Driver> SeesawDeviceInit<D> for NeoSlider<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}

impl<D: Driver> NeoSlider<D> {
    pub fn slider_value(&mut self) -> Result<u16, crate::SeesawError<D::I2cError>> {
        self.analog_read(18)
    }
}

seesaw_device! {
    /// RotaryEncoder
    name: RotaryEncoder,
    hardware_id: HardwareId::SAMD09,
    product_id: 4991,
    default_addr: 0x36,
    modules:  [
        EncoderModule { button_pin: 24 },
        GpioModule,
        NeopixelModule { num_leds: 1, pin: 6 },
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_button())
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}
