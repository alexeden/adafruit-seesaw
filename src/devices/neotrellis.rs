use super::SeesawDeviceInit;
use crate::{
    modules::{keypad::KeypadModule, status::StatusModule, HardwareId},
    seesaw_device, Driver, SeesawError,
};
#[cfg(feature = "module_neopixel")]
use crate::modules::neopixel::NeopixelModule;

seesaw_device! {
    name: NeoTrellis,
    hardware_id: HardwareId::SAMD09,
    product_id: 3954,
    default_addr: 0x2E
}

pub type NeoTrellisColor = rgb::Grb<u8>;

impl<D: Driver> KeypadModule<D> for NeoTrellis<D> {
    const NUM_COLS: u8 = 4;
    const NUM_ROWS: u8 = 4;
}

#[cfg(feature = "module_neopixel")]
impl<D: Driver> NeopixelModule<D> for NeoTrellis<D> {
    type Color = NeoTrellisColor;

    const N_LEDS: usize = 16;
    const PIN: u8 = 3;
}

impl<D: Driver> SeesawDeviceInit<D> for NeoTrellis<D> {
    fn init(mut self) -> Result<Self, SeesawError<D::Error>> {
        self.reset_and_verify_seesaw()?;
        #[cfg(feature = "module_neopixel")]
        self.enable_neopixel()?;
        Ok(self)
    }
}

impl<D: Driver> NeoTrellis<D> {
    pub fn set_xy_neopixel_color(
        &mut self,
        x: u8,
        y: u8,
        color: NeoTrellisColor,
    ) -> Result<(), SeesawError<D::Error>>
    where
        [(); 2 + Self::C_SIZE]: Sized,
    {
        self.set_nth_neopixel_color((y * Self::NUM_COLS + x).into(), color)
    }
}
