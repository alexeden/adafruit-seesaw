#[macro_export]
macro_rules! seesaw_device {
    (
        $(#[$attr:meta])*
        name: $name:ident,
        hardware_id: $hardware_id:expr,
        product_id: $product_id:expr,
        default_addr: $default_addr:expr,
        modules: [ $($module:tt),* ]
    ) => {
        $(#[$attr])*
        ///
        /// [Adafruit Product Page](https://www.adafruit.com/product/$product_id)
        pub struct $name<M>(u8, M);

        impl<D: $crate::driver::Driver> $crate::SeesawDevice<D> for $name<D> {
            type Error = $crate::SeesawError<D::I2cError>;

            const DEFAULT_ADDR: u8 = $default_addr;
            const HARDWARE_ID: u8 = $hardware_id.into();
            const PRODUCT_ID: u16 = $product_id;

            fn addr(&self) -> u8 {
                self.0
            }

            fn driver(&mut self) -> &mut D {
                &mut self.1
            }

            fn new(addr: u8, driver: D) -> Self {
                Self(addr, driver)
            }
        }

        $(
            $crate::impl_device_module! { $name, $module }
        ),*
    };
}

#[macro_export]
macro_rules! impl_device_module {
    ($device:ident, GpioModule) => {
        impl<D: $crate::driver::Driver> $crate::GpioModule<D> for $device<D> {}
    };
}

#[macro_export]
macro_rules! impl_device_encoder_module {
    ($device:ident, button_pin: $button_pin:expr) => {
        impl<D: $crate::driver::Driver> $crate::EncoderModule<D> for $device<D> {
            const ENCODER_BTN_PIN: u8 = $button_pin;
        }
    };
}

// #[macro_export]
// macro_rules! impl_device_gpio_module {
//     ($device:ident) => {
//         impl<D: $crate::driver::Driver> $crate::GpioModule<D> for $device<D>
// {}     };
// }

#[macro_export]
macro_rules! impl_device_neopixel_module {
    ($device:ident, num_leds: $num_leds:expr, pin: $pin:expr) => {
        impl<D: $crate::driver::Driver> $crate::NeopixelModule<D> for $device<D> {
            const N_LEDS: u16 = $num_leds;
            const PIN: u8 = $pin;
        }
    };
}
