# KB2040 Microcontroller Example

## Setup
Install the RP2040 toolchain: 
```bash
rustup target add thumbv6m-none-eabi 
cargo install elf2uf2-rs
```

Plug in the KB2040 and get it into bootloader mode by pressing the reset button while you hold down the boot button.
Run `cargo run` to flash the board. This uses elf2uf2-rs to convert .elf to .uf2 and upload to the board.

## References
[The Rusty Bits: Embedded Rust Setup Explained](https://www.youtube.com/watch?v=TOAynddiu5M)