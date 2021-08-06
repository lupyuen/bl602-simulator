# GPIO Rust Firmware for BL602 IoT SDK

This BL602 firmware shows how we may create Rust firmware with the BL602 IoT SDK. Read the article...

- [__"Rust on RISC-V BL602: Is It Sunny?"__](https://lupyuen.github.io/articles/adc)

Rust source code for the BL602 firmware is here...

- [`rust`: Rust source code](rust)

- [`rust/src/lib.rs`: Rust commands](rust/src/lib.rs)

Run this script to build, flash and run the BL602 Rust firmware...

- [`run.sh`: Build, flash and run BL602 Rust firmware](run.sh)

This script links the compiled Rust code into the BL602 firmware by overwriting the compiled `rust_app` Stub Library...

- [`rust-app`: BL602 Stub Library for Rust Application](../../components/3rdparty/rust-app)

The script uses a Custom Rust Target `riscv32imacf-unknown-none-elf`...

- [`riscv32imacf-unknown-none-elf.json`: Rust Target for BL602](riscv32imacf-unknown-none-elf.json)
