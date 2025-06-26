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

Communicating with Seesaw devices requires a driver that implements both `I2C` traits and `Delay` from `embedded-hal`.

## `#![no_std]` (single-threaded)

## Single Device

If you're interfacing with a single device--i.e. you don't need to share access to a device's requisite I2C bus nor the Delay--create a `SeesawDriver` and provide it to the device constructor:

```rs
// Setup on an STM32F405
let cp = cortex_m::Peripherals::take().unwrap();
let clocks = dp.RCC.constrain().cfgr.freeze();
let delay = cp.SYST.delay(&clocks);
let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
let seesaw = SeesawDriver::new(delay, i2c);
let neokeys = NeoKey1x4::new_with_default_addr(seesaw).init().unwrap();
```

## Multiple Devices

### `no_std`

For multiple devices, use third-party libraries like [`embedded-hal-bus`](https://crates.io/crates/embedded-hal-bus) and [`embassy-time`](https://crates.io/crates/embassy-time) to facilitate bus and delay sharing.

[Complete example here.](https://github.com/alexeden/adafruit-seesaw/blob/main/examples/embedded_hal_rotary_encoder_test.rs)

```rs
// Setup on an STM32F405
use embassy_time::Delay;
use embedded_hal_bus::i2c::RefCellDevice;

let clocks = dp.RCC.constrain().cfgr.freeze();
let i2c = RefCell::new(I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks));

let encoder_driver_1 = SeesawDriver::new(Delay, RefCellDevice::new(&i2c));
let encoder_1 = RotaryEncoder::new(0x00, encoder_driver_1).init().unwrap();

let encoder_driver_2 = SeesawDriver::new(Delay, RefCellDevice::new(&i2c));
let encoder_2 = RotaryEncoder::new(0x01, encoder_driver_2).init().unwrap();
```

### `std`

For multi-threaded use, implementation looks the same as above for `no_std`, but you can use components like [`MutexDevice`](https://docs.rs/embedded-hal-bus/0.3.0/embedded_hal_bus/i2c/struct.MutexDevice.html) from the `embedded-hal-bus` crate.

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
let neokeys = NeoKey1x4::new_with_default_addr(seesaw_driver);
```

### Using a custom address

```rs
let neokeys = NeoKey1x4::new(0x00, seesaw_driver);
```

### Initializing

Devices that implement `SeesawDevice` also implmement `SeesawDeviceInit`, which defines a device-specific `init` function for setting up a device's hardware functionality. The intention is to run a set of sensible defaults so you don't have to remember to do it yourself.

```rs
let neokeys = NeoKey1x4::new_with_default_addr(seesaw_driver)
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

| Device                                               | Product ID | MCU       | Notes                                                                                                   |
| ---------------------------------------------------- | ---------- | --------- | ------------------------------------------------------------------------------------------------------- |
| [ArcadeButton1x4](https://adafruit.com/product/5296) | 5296       | ATTiny8x7 |                                                                                                         |
| [NeoKey1x4](https://adafruit.com/product/4980)       | 4980       | SAMD09    |                                                                                                         |
| [NeoSlider](https://adafruit.com/product/5295)       | 5295       | ATTiny8x7 |                                                                                                         |
| [NeoTrellis](https://adafruit.com/product/3954)      | 3954       | SAMD09    | [Example demo video `neotrellis_ripples.rs`](https://storage.googleapis.com/apemedia/neotrellis576.mp4) |
| [NeoRotary4](https://adafruit.com/product/5752)      | 5752       | ATTiny8x7 |                                                                                                         |
| [RotaryEncoder](https://adafruit.com/product/4991)   | 4991       | SAMD09    |                                                                                                         |

# Creating Your Own Devices

So far, this crate only implements a few Seesaw devices (i.e., the ones that I currently own). You can define your own device using the `seesaw_device!` macro and then configuring its modules using their respective traits.

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
let neokeys = NeoKey2x3::new_with_default_addr(seesaw_driver)
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
