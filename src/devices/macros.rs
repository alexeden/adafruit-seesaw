#[macro_export(local_inner_macros)]
macro_rules! seesaw_device {
    (
        $(#[$attr:meta])*
        name: $name:ident,
        hardware_id: $hardware_id:expr,
        product_id: $product_id:expr,
        default_addr: $default_addr:expr,
        modules: [
            $($module_name:ident $({
                $($const_name:ident: $const_value:expr $(,)?),*
            })?),*
            $(,)?
        ]
         $(,)?
    ) => {
        #[doc=core::concat!("[Product Page](https://www.adafruit.com/product/", core::stringify!($product_id),")")]
        $(#[$attr])*
        #[derive(Debug)]
        pub struct $name<D>(u8, D);

        impl $name<()> {
            pub const fn default_addr() -> u8 {
                $default_addr
            }
            pub const fn hardware_id() -> $crate::modules::HardwareId {
                $hardware_id
            }
            pub const fn product_id() -> u16 {
                $product_id
            }
        }

        impl<D: $crate::Driver> $crate::devices::SeesawDevice for $name<D> {
            type Driver = D;
            type Error = $crate::SeesawError<D::Error>;
            const DEFAULT_ADDR: u8 = $default_addr;
            const HARDWARE_ID: $crate::modules::HardwareId = $hardware_id;
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

            fn new_with_default_addr(driver: D) -> Self {
                Self(Self::DEFAULT_ADDR, driver)
            }
        }

        $(
            impl_device_module! { $name, $module_name $({$($const_name: $const_value),*})* }
        )*
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_device_module {
    ($device:ident, AdcModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::adc::AdcModule<D> for $device<D> {}
    };
    ($device:ident, EncoderModule { button_pin: $button_pin:expr }) => {
        impl<D: $crate::Driver> $crate::modules::encoder::EncoderModule<D> for $device<D> {
            const ENCODER_BTN_PIN: u8 = $button_pin;
        }
    };
    ($device:ident, GpioModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::gpio::GpioModule<D> for $device<D> {}
    };
    ($device:ident, KeypadModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::keypad::KeypadModule<D> for $device<D> {}
    };
    ($device:ident, NeopixelModule { num_leds: $num_leds:expr, pin: $pin:expr }) => {
        impl<D: $crate::Driver> $crate::modules::neopixel::NeopixelModule<D> for $device<D> {
            const N_LEDS: u16 = $num_leds;
            const PIN: u8 = $pin;
        }
    };
    ($device:ident, StatusModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::StatusModule<D> for $device<D> {}
    };
    ($device:ident, TimerModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::timer::TimerModule<D> for $device<D> {}
    };
}
