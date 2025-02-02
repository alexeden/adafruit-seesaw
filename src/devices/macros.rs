#[macro_export(local_inner_macros)]
macro_rules! seesaw_device {
    (
        $(#[$attr:meta])*
        name: $name:ident,
        hardware_id: $hardware_id:expr,
        product_id: $product_id:expr,
        default_addr: $default_addr:expr,
        modules: [
            $($module_name:ident$(<$($module_param_name:ident =$module_param:ty),*>)? $({
                $($const_name:ident: $const_value:expr),*
            })?),*
            $(,)?
        ]
         $(,)?
    ) => {
        #[doc=core::concat!("[Adafruit Product Page](https://www.adafruit.com/product/", core::stringify!($product_id),")")]
        #[doc=core::concat!("")]
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
            impl_device_module! {
                $name, $module_name$(<$($module_param_name = $module_param),*>)? $({
                    $($const_name: $const_value),*
                })*
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_device_module {
    ($device:ident, AdcModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::adc::AdcModule<D> for $device<D> {}
    };
    ($device:ident, EncoderModule {
        num_encoders: $num_encoders:expr,
        encoder_btn_pins: $button_pins:expr
    }) => {
        impl<D: $crate::Driver> $crate::modules::encoder::EncoderModule<D, $num_encoders>
            for $device<D>
        {
            const ENCODER_BTN_PINS: [u8; $num_encoders] = $button_pins;
        }
    };
    ($device:ident, GpioModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::gpio::GpioModule<D> for $device<D> {}
    };
    ($device:ident, KeypadModule { num_cols: $num_cols:expr, num_rows: $num_rows:expr }) => {
        impl<D: $crate::Driver> $crate::modules::keypad::KeypadModule<D> for $device<D> {
            const NUM_COLS: u8 = $num_cols;
            const NUM_ROWS: u8 = $num_rows;
        }
    };
    ($device:ident, NeopixelModule<color_type = $color_type:ty> { num_leds: $num_leds:expr, pin: $pin:expr }) => {
        impl<D: $crate::Driver> $crate::modules::neopixel::NeopixelModule<D> for $device<D> {
            type C = $color_type;

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
    ($device:ident, QuadEncoderModule $({})?) => {
        impl<D: $crate::Driver> $crate::modules::quad_encoder::QuadEncoderModule<D> for $device<D> {}
    };
}
