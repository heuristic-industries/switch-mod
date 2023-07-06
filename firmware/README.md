# Single Switch Guitar Pedal Firmware

This is an adaptation of the firmware used in my own pedals for bypass switching, modified to work with pedals that already have some form of "soft switching."

Features:

- input debouncing
- press to toggle, hold for momentary operation
- persistence of state to EEPROM, with write leveling

## Building

Like any Rust project, `cargo build` will compile the binaries.

Optionally, as a convenience, you can use `cargo run`, which will build and attempt to flash the board using `flash.sh`, which is specific to my toolchain and may need to be tweaked depending on your setup.

### Notes on compilation

Due to an LLVM bug that broke the AVR target, this project requires any nightly after 2022-05-09 (when it [was fixed](https://github.com/rust-lang/rust/pull/96845)).
