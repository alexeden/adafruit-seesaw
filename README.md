# adafruit-seesaw [WIP]

Platform-agnostic driver to communicate with devices that implement the [Adafruit Seesaw firmware.](https://github.com/adafruit/Adafruit_Seesaw)


## TODOs

#### Seesaw-related

_Modules_

Seesaw Module | Implemented
---|---
Adc | []
Dac | []
Dap | []
Eeprom | [x]
Encoder | [x]
Gpio | [x]
Interrupt | []
Keypad | []
Neopixel | [x]
Sercom0 | []
Spectrum | []
Status | [x]
Timer | []
Touch | []

_Devices_

[] Ask Adafruit nicely for a list of their products that use the Seesaw firmware

Device | Implemented
---|---


#### Library/API-related

[] Add feature for enabling eh alpha
[] Add features for using platform-specific mutexes ([these flags will be coupled directly with the feaure flags of `shared-bus`](https://docs.rs/crate/shared-bus/latest/features))


## License
shared-bus is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---


*Not affiliated with nor officially supported by Adafruit.*
