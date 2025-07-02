#[cfg(feature = "device_arcade_button_1x4")]
mod arcade_button_1x4;
mod generic_device;
pub mod macros;
#[cfg(feature = "device_neokey_1x4")]
mod neokey_1x4;
#[cfg(feature = "device_neorotary4")]
mod neorotary4;
#[cfg(feature = "device_neoslider")]
mod neoslider;
#[cfg(feature = "device_neotrellis")]
mod neotrellis;
#[cfg(feature = "device_rotary_encoder")]
mod rotary_encoder;
use crate::{
    modules::{status::StatusModule, HardwareId},
    Driver, SeesawError,
};
#[cfg(feature = "device_arcade_button_1x4")]
pub use arcade_button_1x4::*;
pub use generic_device::*;
#[cfg(feature = "device_neokey_1x4")]
pub use neokey_1x4::*;
#[cfg(feature = "device_neorotary4")]
pub use neorotary4::*;
#[cfg(feature = "device_neoslider")]
pub use neoslider::*;
#[cfg(feature = "device_neotrellis")]
pub use neotrellis::*;
#[cfg(feature = "device_rotary_encoder")]
pub use rotary_encoder::*;

pub trait SeesawDevice {
    type Driver: crate::Driver;

    const DEFAULT_ADDR: u8;
    const HARDWARE_ID: HardwareId;
    const PRODUCT_ID: u16;

    fn addr(&self) -> u8;

    fn driver(&mut self) -> &mut Self::Driver;

    fn new(addr: u8, driver: Self::Driver) -> Self;

    fn new_with_default_addr(driver: Self::Driver) -> Self;
}

/// All devices implement the status module
impl<D: Driver, T: SeesawDevice<Driver = D>> StatusModule<D> for T {}

/// At startup, Seesaw devices typically have a unique set of initialization
/// calls to be made. e.g. for a Neokey1x4, we need to enable the on-board
/// neopixel and also do some pin mode setting to get everything working.
/// All devices implement `DeviceInit` with a set of sensible defaults.
pub trait SeesawDeviceInit<D: Driver>: SeesawDevice<Driver = D>
where
    Self: Sized,
{
    fn init(self) -> Result<Self, SeesawError<D::Error>>;
}
