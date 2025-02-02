use super::SeesawDeviceInit;
use crate::{
    modules::{keypad::KeypadModule, neopixel::NeopixelModule, status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};

seesaw_device! {
    name: NeoTrellis,
    hardware_id: HardwareId::SAMD09,
    product_id: 3954,
    default_addr: 0x2E,
    modules: [
        NeopixelModule<color_type = rgb::Rgb<u8>> { num_leds: 16, pin: 3 },
        KeypadModule { num_cols: 4, num_rows: 4 },
    ]
}

impl<D: Driver> SeesawDeviceInit<D> for NeoTrellis<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .map(|_| self)
    }
}

impl<D: Driver> NeoTrellis<D> {
    pub const fn num_cols(&self) -> u8 {
        Self::NUM_COLS
    }

    pub const fn num_rows(&self) -> u8 {
        Self::NUM_ROWS
    }

    pub fn set_xy_neopixel_color(
        &mut self,
        x: u8,
        y: u8,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), SeesawError<D::Error>> {
        self.set_nth_neopixel_color((y * Self::NUM_COLS + x).into(), r, g, b)
    }
}
