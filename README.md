# adafruit-seesaw [![crates.io page](https://img.shields.io/crates/v/adafruit-seesaw)](https://crates.io/crates/adafruit-seesaw) [![docs.rs](https://docs.rs/adafruit-seesaw/badge.svg)](https://docs.rs/adafruit-seesaw)

Platform-agnostic driver to communicate with devices that implement the [Adafruit Seesaw firmware.](https://github.com/adafruit/Adafruit_Seesaw) See the Seesaw [guide](https://learn.adafruit.com/adafruit-seesaw-atsamd09-breakout) for more information on the firmware.

# Introduction

The library uses and follows the patterns of the [`shared-bus`](https://github.com/Rahix/shared-bus) library so that multiple devices can be connected and communicated with without owning the I2C bus.

Communicating with Seesaw devices requires a bus that implements both `I2C` traits and `Delay` from `embedded-hal`.

# Using within a single thread

If you're communicating with devices within a single thread, use the `SeesawSingleThread` struct, which uses the `NullMutex` bus mutex implementation from `shared-bus:

```rs
// Setup on an STM32F405
let cp = cortex_m::Peripherals::take().unwrap();
let clocks = dp.RCC.constrain().cfgr.freeze();
let delay = cp.SYST.delay(&clocks);
let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
let seesaw = SeesawSingleThread::new(delay, i2c);
```

# Using across multiple threads

Example usage of using multi-threaded `Seesaw` in a `std` context, running on an ESP32-S3:

```rs
use adafruit_seesaw::{prelude::*, RotaryEncoder, Seesaw};
use esp_idf_hal::{
    self,
    delay::Delay,
    gpio::PinDriver,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use shared_bus::{once_cell, I2cProxy};
use std::time::Duration;

type SeesawMultiThread<BUS> = Seesaw<std::sync::Mutex<BUS>>;

fn main() -> ! {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // System
    let peripherals = Peripherals::take().unwrap();
    let mut i2c_power = PinDriver::output(peripherals.pins.gpio7).unwrap();
    i2c_power.set_low().expect("Failed to turn off I2C power");

    // I2C
    let (sda, scl) = (peripherals.pins.gpio3, peripherals.pins.gpio4);
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::<'static>::new(peripherals.i2c0, sda, scl, &config)
        .expect("Failed to create I2C driver");
    i2c_power.set_high().expect("Failed to turn on I2C power");
    std::thread::sleep(Duration::from_millis(50));

    let bus: &'static _ = shared_bus::new_std!(I2cDriver = i2c).unwrap();
    let seesaw: &'static _ = {
        use once_cell::sync::OnceCell;

        static MANAGER: OnceCell<
            Seesaw<
                std::sync::Mutex<
                    adafruit_seesaw::bus::Bus<
                        Delay,
                        I2cProxy<'_, std::sync::Mutex<I2cDriver<'_>>>,
                    >,
                >,
            >,
        > = OnceCell::new();

        let m = SeesawMultiThread::new(Delay, bus.acquire_i2c());
        match MANAGER.set(m) {
            Ok(_) => MANAGER.get(),
            Err(_) => None,
        }
    }
    .unwrap();

    let _encoder =
        RotaryEncoder::new_with_default_addr(seesaw.acquire_driver())
            .init()
            .expect("Failed to start rotary encoder.");

    loop {
        // Do stuff with rotary encoder
    }
}
```


# Creating a Device

All devices implement the `SeesawDevice` trait and have the same constructor function, along with lots of other device-specific information.

| Product value   | Const method on all `SeesawDevice`s | Notes                                                                                  |
| --------------- | ----------------------------------- | -------------------------------------------------------------------------------------- |
| Default Address | `Device::default_addr()`            |
| Hardware ID     | `Device::hardware_id()`             | This value depends on the host MCU of the device                                       |
| Product ID      | `Device::product_id()`              | You can use this value to go to the product page at `adafruit.com/product/$product_id` |

Let's talk to a [NeoKey1x4](https://www.adafruit.com/product/4980) using the `seesaw` manager we created above.

### Using the default address

```rs
let neokeys = NeoKey1x4::new_with_default_addr(seesaw.acquire_driver());
```

### Using a custom address

```rs
let neokeys = NeoKey1x4::new(0x00, seesaw.acquire_driver());
```

# Initializing Devices

Devices that implement `SeesawDevice` also implmement `SeesawDeviceInit`, which defines a device-specific `init` function for setting up a device's hardware functionality. The intention is to run a set of sensible defaults so you don't have to remember to do it yourself.

```rs
let neokeys = NeoKey1x4::new_with_default_addr(seesaw.acquire_driver())
    .init()
    .expect("Failed to initialize NeoKey1x4");
```

For instance, the `init` function for our `Neokey1x4` does the following:

- Resets the device
- Reads & verifies the device hardware ID
- Enables the on-device neopixels
- Enables the on-device buttons

Calling `init` is of course optional, but without it you'll have to handle initialization yourself.

# Creating Your Own Devices

So far, this library only implements a few Seesaw devices (i.e., the ones that I currently own). You can define your own device using the `seesaw_device!` macro.

Let's assume you have some future Adafruit Neokey-esque device that has 6 buttons and 6 neopixels.

```rs
seesaw_device! {
    name: Neokey2x3,
    hardware_id: HardwareId::_,
    product_id: _,
    default_addr: _,
    modules: [
        GpioModule,
        NeopixelModule { num_leds: 6, pin: _ },
    ]
}
```

The last thing you might want to do is implmeent the `SeesawDeviceInit` trait to handle the device intialization:

```rs
impl<D: Driver> SeesawDeviceInit<D> for Neokey2x3<D> {
    fn init(mut self) -> Result<Self, Self::Error> {
        self.reset_and_verify_seesaw()
            .and_then(|_| self.enable_neopixel())
            .and_then(|_| self.enable_button_pins())
            .map(|_| self)
    }
}
```

Now you can use the new device as you would any other:

```rs
let neokeys = NeoKey2x3::new_with_default_addr(seesaw.acquire_driver())
    .init()
    .expect("Failed to initialize NeoKey1x4");
```

# TODOs

### Seesaw-related

_Modules_

| Seesaw Module | Implemented |
| ------------- | ----------- |
| ADC           | ✅          |
| EEPROM        | ⬜️         |
| Encoder       | ✅          |
| GPIO          | ✅          |
| Keypad        | ⬜️         |
| Neopixel      | ✅          |
| Sercom0       | ⬜️         |
| Spectrum      | ⬜️         |
| Status        | ✅          |
| Timer         | ✅          |
| Touch         | ⬜️         |

_Devices_

- ⬜️ Ask Adafruit nicely for a list of their products that use the Seesaw firmware

| Device                                               | Product ID | MCU       | Implemented |
| ---------------------------------------------------- | ---------- | --------- | ----------- |
| [ArcadeButton1x4](https://adafruit.com/product/5296) | 5296       | ATTiny8x7 | ✅          |
| [NeoKey1x4](https://adafruit.com/product/4980)       | 4980       | SAMD09    | ✅          |
| [NeoSlider](https://adafruit.com/product/5295)       | 5295       | ATTiny8x7 | ✅          |
| [RotaryEncoder](https://adafruit.com/product/4991)   | 4991       | SAMD09    | ✅          |

### Library/API-related

- ⬜️ Add feature flag and implementations for using eh alpha
- ⬜️ Add features for using platform-specific mutexes ([these flags will be coupled directly with the feaure flags of `shared-bus`](https://docs.rs/crate/shared-bus/latest/features))

- ⬜️ Setup github actions for CI porpoises

# License

adafruit-seesaw is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

_Not affiliated with, nor officially supported by Adafruit._
