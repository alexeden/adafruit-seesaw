# adafruit-seesaw [WIP]

Platform-agnostic driver to communicate with devices that implement the [Adafruit Seesaw firmware.](https://github.com/adafruit/Adafruit_Seesaw)

## TODOs

### Seesaw-related

_Modules_

| Seesaw Module | Implemented          |
| ------------- | -------------------- |
| Adc           | :white_large_square: |
| Dac           | :white_large_square: |
| Dap           | :white_large_square: |
| Eeprom        | :white_check_mark:   |
| Encoder       | :white_check_mark:   |
| Gpio          | :white_check_mark:   |
| Interrupt     | :white_large_square: |
| Keypad        | :white_large_square: |
| Neopixel      | :white_check_mark:   |
| Sercom0       | :white_large_square: |
| Spectrum      | :white_large_square: |
| Status        | :white_check_mark:   |
| Timer         | :white_large_square: |
| Touch         | :white_large_square: |

_Devices_

* :white_large_square: Ask Adafruit nicely for a list of their products that use the Seesaw firmware

| Device | Implemented |
| ------ | ----------- |

### Library/API-related

* :white_large_square: Add feature flag and implementations for using eh alpha
* :white_large_square: Add features for using platform-specific mutexes ([these flags will be coupled directly with the feaure flags of `shared-bus`](https://docs.rs/crate/shared-bus/latest/features))

* :white_large_square: Setup github actions for CI porpoises

## License

shared-bus is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

_Not affiliated with nor officially supported by Adafruit._
