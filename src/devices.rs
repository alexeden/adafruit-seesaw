use crate::{driver::Driver, modules::*, seesaw_device, HardwareId, SeesawDeviceInit};

/// All devices implement the status module
impl<D: Driver, T: super::SeesawDevice<D>> StatusModule<D> for T {}

seesaw_device! {
    #[doc(hidden)]
    name: GenericDevice,
    hardware_id: HardwareId::SAMD09,
    product_id: 0,
    default_addr: 0x49,
    modules: []
}

impl<D: Driver> SeesawDeviceInit<D> for GenericDevice<D> {
    fn init(&mut self) -> Result<(), Self::Error> {
        self.reset()
    }
}

seesaw_device! {
    /// ArcadeButton1x4
    name: ArcadeButton1x4,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5296,
    default_addr: 0x3A,
    modules: [
        GpioModule,
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for ArcadeButton1x4<D> {
    fn init(&mut self) -> Result<(), Self::Error> {
        self.reset()
        // self.reset_and_verify_seesaw()
        // Ok(())
        // .and_then(|_| self.enable_button_pins())
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
    fn init(&mut self) -> Result<(), Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button_pins())
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
        GpioModule,
        NeopixelModule { num_leds: 4, pin: 14},
    ]
);

impl<D: Driver> SeesawDeviceInit<D> for NeoSlider<D> {
    fn init(&mut self) -> Result<(), Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
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
    fn init(&mut self) -> Result<(), Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_button())
            .and_then(|_| self.enable_neopixel())
    }
}
