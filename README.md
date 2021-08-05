# bl602-wasm: BL602 / BL604 Simulator in WebAssembly

Let's __Simulate BL602 / BL604 Rust Firmware__ in a Web Browser with __WebAssembly__...

1.  We take this BL602 / BL604 __Blinky Firmware in Rust__...

    - [__`sdk_app_rust_gpio`__](https://github.com/lupyuen/bl_iot_sdk/tree/master/customer_app/sdk_app_rust_gpio)

1.  Which calls the __Rust Wrapper for BL602 IoT SDK__...

    - [__Rust Wrapper for BL602 IoT SDK__](https://crates.io/crates/bl602-sdk)

1.  We __compile to WebAssembly__ the Rust Firmware and Rust Wrapper

1.  Replace calls to BL602 IoT SDK by __Stub Functions__

    (Like for the BL602 GPIO HAL)

1.  Add a __HTML + JavaScript UI__ to simulate the BL602 / BL602 Board...

    - [__“Simulate RISC-V BL602 with WebAssembly, uLisp and Blockly”__](https://lupyuen.github.io/articles/wasm)
    
    (Without the Blockly part, since we can't compile Rust in a Web Browser)
