# BL602 / BL604 Rust Scripting Library

Based on Rhai Scripting Engine: https://rhai.rs/book

[Follow the updates in this Twitter Thread](https://twitter.com/MisterTechBlog/status/1427758328004759552)

Here's a Rhai Script that works with BL602 Simulator: [src/lib.rs](src/lib.rs)

```rust
//  Blink the LED:
//  PineCone Blue LED is connected on BL602 GPIO 11
let LED_GPIO = 11;

//  Configure the LED GPIO for output (instead of input)
gpio::enable_output(LED_GPIO, 0, 0);

//  Blink the LED 5 times
for i in range(0, 10) {

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    gpio::output_set(
        LED_GPIO, 
        i % 2
    );

    //  Sleep 1 second
    time_delay(1000);
}
```

Compare the above Rhai Script with the equivalent Rust Firmware: [sdk_app_rust_gpio/rust/src/lib.rs](../sdk_app_rust_gpio/rust/src/lib.rs)

```rust
//  PineCone Blue LED is connected on BL602 GPIO 11
const LED_GPIO: u8 = 11;  //  `u8` is 8-bit unsigned integer

//  Configure the LED GPIO for output (instead of input)
gpio::enable_output(LED_GPIO, 0, 0)        //  No pullup, no pulldown
    .expect("GPIO enable output failed");  //  Halt on error

//  Blink the LED 5 times
for i in 0..10 {  //  Iterates 10 times from 0 to 9 (`..` excludes 10)

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    gpio::output_set(  //  Set the GPIO output (from BL602 GPIO HAL)
        LED_GPIO,      //  GPIO pin number
        i % 2          //  0 for low, 1 for high
    ).expect("GPIO output failed");  //  Halt on error

    //  Sleep 1 second
    time_delay(                   //  Sleep by number of ticks (from NimBLE Porting Layer)
        time_ms_to_ticks32(1000)  //  Convert 1,000 milliseconds to ticks (from NimBLE Porting Layer)
    );
}
```

The Rhai Scripting Engine shall be integrated with Blockly to allow drag-and-drop scripting: https://github.com/lupyuen2/blockly-bl602

To run Rhai Scripts on BL602, we shall transcode the script to uLisp: https://lupyuen.github.io/articles/rustsim#appendix-rhai-scripts-on-bl602

# Transcode Rhai Script to uLisp

[Follow the updates in this Twitter Thread](https://twitter.com/MisterTechBlog/status/1427758328004759552)

Transcoder is located here: [src/transcode.rs](src/transcode.rs)

This code from [src/lib.rs](src/lib.rs)...

```rust
//  Compile Rhai Script to an Abstract Syntax Tree
let ast = engine.compile(script)
    .unwrap();
println!("AST: {:#?}", ast);

//  Transcode the Rhai Abstract Syntax Tree to uLisp
transcode::transcode(&ast);
```

Compiles this Rhai Script...

```rust
//  Blink the LED:
//  PineCone Blue LED is connected on BL602 GPIO 11
let LED_GPIO = 11;

//  Configure the LED GPIO for output (instead of input)
gpio::enable_output(LED_GPIO, 0, 0);

//  Blink the LED 5 times
for i in range(0, 10) {

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    gpio::output_set(
        LED_GPIO, 
        i % 2
    );

    //  Sleep 1 second
    time_delay(1000);
}
```

To the Abstract Syntax Tree (AST) below. Then we walk the AST nodes and transcode them to uLisp.

```text
AST {
    source: None,
    body: Block[
        Var(
            11 @ 4:24,
            "LED_GPIO" @ 4:13,
            (),
            4:9,
        ),
        FnCall(
            FnCallExpr {
                namespace: Some(
                    gpio,
                ),
                hashes: 14026165341011925297,
                args: [
                    Variable(LED_GPIO #1) @ 7:29,
                    StackSlot(0) @ 7:39,
                    StackSlot(1) @ 7:42,
                ],
                constants: [
                    0,
                    0,
                ],
                name: "enable_output",
                capture: false,
            },
            7:15,
        ),
        For(
            FnCall {
                name: "range",
                hash: 6483209994621034098,
                args: [
                    StackSlot(0) @ 10:24,
                    StackSlot(1) @ 10:27,
                ],
                constants: [
                    0,
                    10,
                ],
            } @ 10:18,
            (
                "i" @ 10:13,
                None,
                Block[
                    FnCall(
                        FnCallExpr {
                            namespace: Some(
                                gpio,
                            ),
                            hashes: 14851760529339133429,
                            args: [
                                Variable(LED_GPIO #2) @ 14:17,
                                FnCall {
                                    name: "%",
                                    hash: 16068100750815511651 (native only),
                                    args: [
                                        Variable(i #1) @ 15:17,
                                        StackSlot(0) @ 15:21,
                                    ],
                                    constants: [
                                        2,
                                    ],
                                } @ 15:19,
                            ],
                            constants: [],
                            name: "output_set",
                            capture: false,
                        },
                        13:19,
                    ),
                    FnCall(
                        FnCallExpr {
                            namespace: None,
                            hashes: 5888125028643394501,
                            args: [
                                StackSlot(0) @ 19:24,
                            ],
                            constants: [
                                1000,
                            ],
                            name: "time_delay",
                            capture: false,
                        },
                        19:13,
                    ),
                ] @ 10:31,
            ),
            10:9,
        ),
        Var(
            40 @ 23:17,
            "a" @ 23:13,
            (),
            23:9,
        ),
        Var(
            2 @ 24:17,
            "b" @ 24:13,
            (),
            24:9,
        ),
        FnCall(
            FnCallExpr {
                namespace: None,
                hashes: 45565345930656002 (native only),
                args: [
                    Variable(a #2) @ 25:9,
                    Variable(b #1) @ 25:13,
                ],
                constants: [],
                name: "+",
                capture: false,
            },
            25:11,
        ),
    ],
    functions: Module,
    resolver: None,
}
```