use crate::{device::DeviceInit, seesaw_device, GpioModule, NeopixelModule, StatusModule};

// GenericDevice
seesaw_device!(GenericDevice);

impl<D: crate::Driver> DeviceInit<D> for GenericDevice<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}

// NeoKey1x4
seesaw_device!(NeoKey1x4);
impl<D: crate::Driver> GpioModule<D> for NeoKey1x4<D> {}
impl<D: crate::Driver> NeopixelModule<D> for NeoKey1x4<D> {
    const N_LEDS: u16 = 4;
    const PIN: u8 = 3;
}
impl<D: crate::Driver> DeviceInit<D> for NeoKey1x4<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_begin()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.set_pin_mode_bulk(NEOKEY_1X4_PINMASK, PinMode::InputPullup))
            .map(|_| self)

        // self.reset().map(|_| self)
    }
}

// NeoSlider
seesaw_device!(NeoSlider);

impl<D: crate::Driver> DeviceInit<D> for NeoSlider<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}

// RotaryEncoder
seesaw_device!(RotaryEncoder);

impl<D: crate::Driver> DeviceInit<D> for RotaryEncoder<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset().map(|_| self)
    }
}
