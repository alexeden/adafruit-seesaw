# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project (hopefully) adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### To Be Added

- Digital write functions within the Seesaw GPIO module [PR here](https://github.com/alexeden/adafruit-seesaw/pull/8)
- [Add definition for the NeoDriver meopixel driver board](https://www.adafruit.com/product/5766)

## [0.8.0] - 2025-01-31

### Added

- Add support for the `keypad` Seesaw module
- Add `NeoTrellis` device definition for the [NeoTrellis 4x4 Keypad](https://www.adafruit.com/product/3954)
- Add NeoTrellis example similar to the [`ripples.ino` example](https://github.com/adafruit/Adafruit_Seesaw/blob/master/examples/NeoTrellis/ripples/ripples.ino)
  - [Demo video here](https://storage.googleapis.com/apemedia/neotrellis576.mp4)

## [0.7.0] - 2025-01-15

### Added

- Add `QuadEncoder` module
- Add device definition for the [QuadRotaryEncoder](https://www.adafruit.com/product/5297) ([@HyperSuperMetaCtrl](https://github.com/HyperSuperMetaCtrl))

## [0.6.1] - 2024-01-29

### Changed

- Fix issues caused by extraneous byte written when setting neopixel color [PR here](https://github.com/alexeden/adafruit-seesaw/pull/5)

## [0.6.0] - 2024-01-29

### Added

- Feature `std`, which provides a std `Mutex` for handling devices across threads
- Trait `BusMutex` which replaces the `BusMutex` trait previously provided by the `shared_bus` dependency
- Struct `RefCellBus`, which implements `BusMutex` and wraps the bus in a `RefCell`
- Partially-type struct `SeesawStdMutex` for creating a thread-save `Seesaw`

### Changed

- **BREAKING** Upgrade `embedded-hal` to `1.0.0`
- **BREAKING** Rename `SeesawSingleThread` --> `SeesawRefCell`; API remains the same
- Split `seesaw_device! { ... }` definitions into their own files under `/src/devices`

### Removed

- Remove dependency on `shared_bus`; the crate now just defines its own `BuxMutex` trait
- Removed several internal intermediate structs, traits, and associated types that are no longer needed because of the single `I2c` trait in eh-1.0
