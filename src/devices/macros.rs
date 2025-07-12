#[macro_export(local_inner_macros)]
macro_rules! seesaw_device {
    (
        $(#[$attr:meta])*
        name: $name:ident,
        hardware_id: $hardware_id:expr,
        product_id: $product_id:expr,
        default_addr: $default_addr:expr
    ) => {
        #[doc=core::concat!("[Adafruit Product Page](https://www.adafruit.com/product/", core::stringify!($product_id),")")]
        #[doc=core::concat!("")]
        $(#[$attr])*
        #[derive(Debug)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    };
}
