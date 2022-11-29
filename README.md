# adafruit-seesaw [WIP]

Platform-agnostic driver to communicate with devices that implement the [Adafruit Seesaw firmware.](https://github.com/adafruit/Adafruit_Seesaw) [(guide)](https://learn.adafruit.com/adafruit-seesaw-atsamd09-breakout)

## TODOs

### Seesaw-related

_Modules_

| Seesaw Module | Implemented |
| ------------- | ----------- |
| ADC           | ⬜️         |
| DAC           | ⬜️         |
| DAP           | ⬜️         |
| EEPROM        | ✅          |
| Encoder       | ✅          |
| GPIO          | ✅          |
| Interrupt     | ⬜️         |
| Keypad        | ⬜️         |
| Neopixel      | ✅          |
| Sercom0       | ⬜️         |
| Spectrum      | ⬜️         |
| Status        | ✅          |
| Timer         | ⬜️         |
| Touch         | ⬜️         |

_Devices_

- ⬜️ Ask Adafruit nicely for a list of their products that use the Seesaw firmware

| Device                                               | Product ID | MCU       | Implemented                                 |
| ---------------------------------------------------- | ---------- | --------- | ------------------------------------------- |
| [ArcadeButton1x4](https://adafruit.com/product/5296) | 5296       | ATTiny8x7 | ⬜️                                         |
| [NeoKey1x4](https://adafruit.com/product/4980)       | 4980       | SAMD09    | ✅                                          |
| [NeoSlider](https://adafruit.com/product/5295)       | 5295       | ATTiny8x7 | ❓ (impl'd, but refuses to work in example) |
| [RotaryEncoder](https://adafruit.com/product/4991)   | 4991       | SAMD09    | ✅                                          |

### Library/API-related

- ⬜️ Add feature flag and implementations for using eh alpha
- ⬜️ Add features for using platform-specific mutexes ([these flags will be coupled directly with the feaure flags of `shared-bus`](https://docs.rs/crate/shared-bus/latest/features))

- ⬜️ Setup github actions for CI porpoises

## License

shared-bus is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

_Not affiliated with nor officially supported by Adafruit._
