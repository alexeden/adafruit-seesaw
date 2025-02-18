![Adafruit Seesaw Logo](/docs/seesaw-logo.png)

[![crates.io page](https://img.shields.io/crates/v/adafruit-seesaw)](https://crates.io/crates/adafruit-seesaw)
[![docs.rs](https://docs.rs/adafruit-seesaw/badge.svg)](https://docs.rs/adafruit-seesaw)
[![CI](https://github.com/alexeden/adafruit-seesaw/actions/workflows/ci.yml/badge.svg)](https://github.com/alexeden/adafruit-seesaw/actions/workflows/ci.yml)

# Introduction

What is Seesaw? [From Adafruit's guide:](https://learn.adafruit.com/adafruit-seesaw-atsamd09-breakout) "Adafruit seesaw is a near-universal converter framework which allows you to add and extend hardware support to any I2C-capable microcontroller or microcomputer. Instead of getting separate I2C GPIO expanders, ADCs, PWM drivers, etc, seesaw can be configured to give a wide range of capabilities."

This crate aims to be a functionally-equivalent Rust driver for [Adafruit's own C++ driver](https://github.com/adafruit/Adafruit_Seesaw).

**A note on terminology:** Adafruit's Seesaw firmware refers to blocks of device functionality/capabilities as "modules". e.g. [a Seesaw device that has controllable neopixels](https://www.adafruit.com/product/4980) will have the [`NeoPixel` module](https://learn.adafruit.com/adafruit-seesaw-atsamd09-breakout/gpio) loaded and available in its firmware. Don't confuse them with Rust modules! This crate exports a module called `modules` that contains all the modules for the Seesaw devices.

If you want to learn more about modules, [this page in the Seesaw guide](https://learn.adafruit.com/adafruit-seesaw-atsamd09-breakout/reading-and-writing-data) explains it pretty well.

# Usage

The library follows the patterns of the [`shared-bus`](https://github.com/Rahix/shared-bus) library so that multiple devices can be connected and communicated with without owning the I2C bus.

Communicating with Seesaw devices requires a bus that implements both `I2C` traits and `Delay` from `embedded-hal`.

## `#![no_std]` (single-threaded)

If you're communicating with devices within a single thread, use the `SeesawRefCell` typed struct, which uses the `RefCellBus` wrapper to enable sharing of the bus across multiple Seesaw devices.

```rs
// Setup on an STM32F405
let cp = cortex_m::Peripherals::take().unwrap();
let clocks = dp.RCC.constrain().cfgr.freeze();
let delay = cp.SYST.delay(&clocks);
let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
let seesaw = SeesawRefCell::new(delay, i2c);
let mut neokeys = NeoKey1x4::new_with_default_addr(seesaw.acquire_driver())
    .init()
    .expect("Failed to start NeoKey1x4");
```

## `std` (multi-threaded)

> This requires turning on the `std` feature flag.

For multi-threaded purposes, use the `SeesawStdMutex` typed struct, which wraps the bus in a std `Mutex`.

Example usage of using multi-threaded `Seesaw` in a `std` context, running on an ESP32-S3:

```rs
use adafruit_seesaw::{devices::RotaryEncoder, prelude::*, SeesawStdMutex};
use esp_idf_hal::{
    self,
    delay::Delay,
    gpio::PinDriver,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use std::time::Duration;

fn main() -> Result<(), anyhow::Error> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // System
    let peripherals = Peripherals::take().unwrap();
    let mut i2c_power = PinDriver::output(peripherals.pins.gpio7).unwrap();
    i2c_power.set_low()?;
    std::thread::sleep(Duration::from_millis(333));

    // I2C
    let (sda, scl) = (peripherals.pins.gpio3, peripherals.pins.gpio4);
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
    i2c_power.set_high()?;
    std::thread::sleep(Duration::from_millis(333));

    let seesaw: &'static _ = {
        use once_cell::sync::OnceCell;
        static MANAGER: OnceCell<SeesawStdMutex<(Delay, I2cDriver<'_>)>> =
            OnceCell::new();

        match MANAGER.set(SeesawStdMutex::new(Delay::new_default(), i2c)) {
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

# Communicating with a Device

At a minimum, a device implements the `SeesawDevice` trait which specifies a common constructor function, along with lots of other device-specific information specified as `const` values:

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

### Initializing

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

# Predefined Devices

The crate comes with a few predefined devices that you can use. [Their documentation is available here.](https://docs.rs/adafruit-seesaw/latest/adafruit_seesaw/devices/index.html)

| Device                                                 | Product ID | MCU       | Notes                                                                                                   |
| ------------------------------------------------------ | ---------- | --------- | ------------------------------------------------------------------------------------------------------- |
| [ArcadeButton1x4](https://adafruit.com/product/5296)   | 5296       | ATTiny8x7 |                                                                                                         |
| [NeoKey1x4](https://adafruit.com/product/4980)         | 4980       | SAMD09    |                                                                                                         |
| [NeoSlider](https://adafruit.com/product/5295)         | 5295       | ATTiny8x7 |                                                                                                         |
| [NeoTrellis](https://adafruit.com/product/3954)        | 3954       | SAMD09    | [Example demo video `neotrellis_ripples.rs`](https://storage.googleapis.com/apemedia/neotrellis576.mp4) |
| [NeoRotary4](https://adafruit.com/product/5752)        | 5752       | ATTiny8x7 |                                                                                                         |
| [RotaryEncoder](https://adafruit.com/product/4991)     | 4991       | SAMD09    |                                                                                                         |


# Creating Your Own Devices

So far, this library only implements a few Seesaw devices (i.e., the ones that I currently own). You can define your own device using the `seesaw_device!` macro and then configuring its modules using their respective traits.

Let's assume you have some future Adafruit Neokey-esque device that has 6 buttons and 6 neopixels.

You call the `seesaw_device!` macro with information about the device:
```rs
seesaw_device! {
    name: Neokey2x3,
    hardware_id: HardwareId::_,
    product_id: _,
    default_addr: _
}
```

Then implement the module traits for its various capabilities:

```rs
impl<D: Driver> GpioModule<D> for Neokey2x3<D> {}
impl<D: Driver> NeopixelModule<D> for Neokey2x3<D> {
    type Color = Neokey2x3Color;

    const N_LEDS: usize = 6;
    const PIN: u8 = _;
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
    .expect("Failed to initialize NeoKey2x3");
```

# Implementation Progress

| Seesaw Module | Implemented |
| ------------- | ----------- |
| ADC           | ✅          |
| EEPROM        | ⬜️         |
| Encoder       | ✅          |
| GPIO          | ✅          |
| Keypad        | ✅          |
| Neopixel      | ✅          |
| Sercom0       | ⬜️         |
| Spectrum      | ⬜️         |
| Status        | ✅          |
| Timer         | ✅          |
| Touch         | ⬜️         |

# Known Issues

These issues are based solely on my own experience testing this crate on my own devices. Any confirmation of similar or contrary experience is much appreciated.

### `.version()` returns an incorrect product ID for some devices

The `.version()` function--a function of the `Status` module--returns incorrect dates and product IDs for some devices. I'm not too concerned about the date, but product ID matters in the case of identifying a device.

| Works for       | Returns                                                      |
| --------------- | ------------------------------------------------------------ |
| `NeoKey1x4`     | `SeesawVersion { id: 4980, year: 2036, month: 5, day: 5 }`   |
| `NeoRotary4`    | `SeesawVersion { id: 5752, year: 2023, month: 6, day: 27 }`  |
| `NeoSlider`     | `SeesawVersion { id: 5295, year: 2021, month: 11, day: 16 }` |
| `RotaryEncoder` | `SeesawVersion { id: 4991, year: 2035, month: 5, day: 5 }`   |

| Does not work for | Returns                                                     | Should be |
| ----------------- | ----------------------------------------------------------- | --------- |
| `NeoTrellis`      | `SeesawVersion { id: 0, year: 2050, month: 10, day: 4 }`    | 3954      |
| `Neopixel Driver` | `SeesawVersion { id: 5742, year: 2023, month: 5, day: 20 }` | 5766      |

# License

adafruit-seesaw is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

_Not affiliated with, nor officially supported by Adafruit._
