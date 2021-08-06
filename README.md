# BL602 / BL604 Simulator in WebAssembly

Follow the updates on Twitter: https://twitter.com/MisterTechBlog/status/1423169766080933891

Let's __Simulate BL602 / BL604 Rust Firmware__ in a Web Browser with __WebAssembly__...

1.  We take this BL602 / BL604 __Blinky Firmware in Rust__...

    - [__`sdk_app_rust_gpio`__](sdk_app_rust_gpio/rust/src/lib.rs)

1.  Which calls the __Rust Wrapper for BL602 IoT SDK__...

    - [__Rust Wrapper for BL602 IoT SDK__](https://crates.io/crates/bl602-sdk)

1.  We __compile to WebAssembly__ the Rust Firmware and Rust Wrapper

1.  In WebAssembly we __intercept calls to BL602 IoT SDK__ with __Stub Functions__

    (Like for the BL602 GPIO HAL)

1.  Add a __Simulator UI (HTML + JavaScript)__ to simulate a __PineCone BL602__ or __PineDio Stack BL604__...

    - [__“Simulate RISC-V BL602 with WebAssembly, uLisp and Blockly”__](https://lupyuen.github.io/articles/wasm)
    
    (Without the Blockly part, since we can't compile Rust in a Web Browser)
    
Why do this in __Rust__?

- Because we have already __parsed the BL602 IoT SDK interfaces__ with `bindgen`

  (While creating the BL602 Rust Wrapper) 

- Which lets us __manipulate the BL602 SDK interfaces__ with Rust in interesting ways

  (Like our `safe_wrap` Procedural Macro in Rust)
    
- More about __BL602 Rust Wrapper__...

  - [__"Rust on RISC-V BL602: Is It Sunny?"__](https://lupyuen.github.io/articles/adc)

Why are we doing this? What __problem are we solving__?

1.  Shorten the __Code - Build - Flash - Test Cycle__ for BL602 and BL604

    (Because flashing BL602 via UART is kinda cumbersome)
    
1.  We could potentially catch __BL602 SDK Calling Errors__ for new devs and __explain the errors in a friendly way__

    (Invalid parameters or usage, like reading a GPIO Pin configured for output)

We might be able to __Simulate C Firmware__ too, if we...
    
- Tweak the BL602 C Firmware to __build with Emscripten__

- And call the __Stub Functions__

# Build BL602 Rust Firmware for WebAssembly

To compile BL602 Rust Firmware into WebAssembly...

```bash
cd sdk_app_rust_gpio/rust
cargo build --target wasm32-unknown-emscripten
# Produces the library file target/wasm32-unknown-emscripten/debug/libapp.a
```

# Build Log

```text
Compiling proc-macro2 v1.0.28
Compiling unicode-xid v0.2.2
Compiling memchr v2.4.0
Compiling syn v1.0.74
Compiling heapless v0.7.3
Compiling cty v0.2.1
Compiling lazy_static v1.4.0
Compiling rustc-serialize v0.3.24
Compiling cstr_core v0.2.4
Compiling quote v1.0.9
Compiling bl602-macros v0.0.2
Compiling bl602-sdk v0.0.6
Compiling app v0.0.1 (/mnt/c/pinecone/bl602-simulator/sdk_app_rust_gpio/rust)
Finished dev [unoptimized + debuginfo] target(s) in 49.48s
```
