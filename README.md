# BL602 / BL604 Simulator in WebAssembly

__Try it here__: https://lupyuen.github.io/bl602-simulator/

__Follow the updates on Twitter__: https://twitter.com/MisterTechBlog/status/1423169766080933891

![BL602 Simulator in WebAssembly](https://lupyuen.github.io/images/adc-simulator2.png)

Let's __Simulate BL602 / BL604 Rust Firmware__ in a Web Browser with __WebAssembly__...

1.  We take this BL602 / BL604 __Blinky Firmware in Rust__...

    - [__Rust Blinky Firmware for BL602__](sdk_app_rust_gpio/rust/src/lib.rs)

1.  Which calls the __Rust Wrapper for BL602 IoT SDK__...

    - [__Rust Wrapper for BL602 IoT SDK__](https://crates.io/crates/bl602-sdk)

1.  We __compile to WebAssembly__ the Rust Firmware and Rust Wrapper

1.  In WebAssembly we __intercept calls to BL602 IoT SDK__ with __Stub Functions__

    (Like for the BL602 GPIO HAL)

    - [__Stub Functions for BL602 Simulator__](bl602-simulator/src/lib.rs)

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

1.  __Automated Testing__ of BL602 Firmware

We might be able to __Simulate C Firmware__ too, if we...
    
- Tweak the BL602 C Firmware to __build with Emscripten__

- And call the __Stub Functions__

# Build BL602 Rust Firmware for WebAssembly

To compile BL602 Rust Firmware into WebAssembly...

```bash
# Download source code
git clone --recursive https://github.com/lupyuen/bl602-simulator
cd bl602-simulator

# Compile the BL602 Rust Firmware
pushd sdk_app_rust_gpio/rust
cargo build --target wasm32-unknown-emscripten
# Produces the library file target/wasm32-unknown-emscripten/debug/libapp.a
popd

# Compile the BL602 Rust Simulator Library
pushd bl602-simulator
cargo build --target wasm32-unknown-emscripten
# Produces the library file target/wasm32-unknown-emscripten/debug/libbl602_simulator.a
popd

# Link the BL602 Rust Firmware and BL602 Rust Simulator Library with Emscripten
. ~/emsdk/emsdk_env.sh
make -f wasm.mk
```

To run the BL602 Simulator, start a Local Web Server and browse to __`docs/wasm.html`__

# Build Log

Compile the BL602 Rust Firmware...

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

Compile the BL602 Rust Simulator Library...

```text
Compiling proc-macro2 v1.0.28
Compiling unicode-xid v0.2.2
Compiling syn v1.0.74
Compiling serde_derive v1.0.127
Compiling serde v1.0.127
Compiling ryu v1.0.5
Compiling serde_json v1.0.66
Compiling itoa v0.4.7
Compiling quote v1.0.9
Compiling bl602-simulator v0.0.1 (/mnt/c/pinecone/bl602-simulator/bl602-simulator) 
Finished dev [unoptimized + debuginfo] target(s) in 1m 08s
```

Link the BL602 Rust Firmware with Emscripten...

```text
emcc -o wasm/wasm.html \
-Wl,--start-group \
sdk_app_rust_gpio/rust/target/wasm32-unknown-emscripten/debug/libapp.a bl602-simulator/target/wasm32-unknown-emscripten/debug/libbl602_simulator.a \
wasm/wasm.o \
-Wl,--end-group \
-g -I include -s WASM=1 -s "EXPORTED_FUNCTIONS=[ '_rust_main', '_clear_simulation_events', '_get_simulation_events' ]" -s "EXTRA_EXPORTED_RUNTIME_METHODS=[ 'cwrap', 'allocate', 'intArrayFromString', 'UTF8ToString' ]" \

cp wasm/wasm.js   docs
cp wasm/wasm.wasm docs
```
