# KB2040 Mini I2C Gamepad QT Example

This example logs the full Adafruit Mini I2C Gamepad QT state over USB serial.

Connect the gamepad to the KB2040 STEMMA QT connector. The example uses the
default gamepad I2C address, `0x50`.

## Setup
Install the RP2040 toolchain: 
```bash
rustup target add thumbv6m-none-eabi 
cargo install elf2uf2-rs
```

Plug in the KB2040 and get it into bootloader mode by pressing the reset button
while you hold down the boot button. Run `cargo run` to flash the board.
This uses elf2uf2-rs to convert .elf to .uf2 and upload to the board.

After flashing, open the KB2040 USB serial port to view joystick and button
state logs. For example, on Linux you can use `screen` in a separate terminal:
- Install `screen`: `sudo apt install screen`
- Open the serial port: `screen /dev/ttyACM0 115200`
- Press the gamepad buttons and observe the logs
- Exit with `Ctrl+A` then `K` then `Y`

## References
[The Rusty Bits: Embedded Rust Setup Explained](https://www.youtube.com/watch?v=TOAynddiu5M)