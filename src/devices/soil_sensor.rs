use super::SeesawDeviceInit;
use crate::{
    modules::{
        gpio::{GpioModule, PinMode},
        neopixel::NeopixelModule,
        status::StatusModule,
        HardwareId,
    },
    seesaw_device, Driver, SeesawError,
};

seesaw_device!(
    name: SoilSensor,
    hardware_id: HardwareId::SAMD09,
    product_id: 4026,
    default_addr: 0x36
);