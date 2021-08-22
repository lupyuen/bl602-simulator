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

![Transcode Rhai Script to uLisp](https://lupyuen.github.io/images/rhai-transcode.jpg)

[Follow the updates in this Twitter Thread](https://twitter.com/MisterTechBlog/status/1427758328004759552)

Transcoder is located here: [src/transcode.rs](src/transcode.rs)

This code from [src/lib.rs](src/lib.rs)...

```rust
//  Compile Rhai Script to an Abstract Syntax Tree
let ast = engine.compile(script)
    .unwrap();

//  Transcode the Rhai Abstract Syntax Tree to uLisp
transcode::transcode(&ast);
```

Compiles this Rhai Script...

```rust
//  Testing Loop
loop { 
    let a = 1;
    print(a);
    if a == 1 { break; }
}

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

//  Evaluate an expression
let a = 40; 
let b = 2;
a + b 
```

To the Abstract Syntax Tree (AST) below. Then we walk the AST nodes...

```text
AST {
    source: None,
    body: Block[
        While(
            (),
            Block[
                Var(
                    1 @ 4:21,
                    "a" @ 4:17,
                    (),
                    4:13,
                ),
                FnCall(
                    FnCallExpr {
                        namespace: None,
                        hashes: 3315725007120985046,
                        args: [
                            Variable(a #1) @ 5:19,
                        ],
                        constants: [],
                        name: "print",
                        capture: false,
                    },
                    5:13,
                ),
                If(
                    FnCall {
                        name: "==",
                        hash: 11715956606336702561 (native only),
                        args: [
                            Variable(a #1) @ 6:16,
                            StackSlot(0) @ 6:21,
                        ],
                        constants: [
                            1,
                        ],
                    } @ 6:18,
                    (
                        Block[
                            Break(
                                6:25,
                            ),
                        ] @ 6:23,
                        Block[],
                    ),
                    6:13,
                ),
            ] @ 3:14,
            3:9,
        ),
        Var(
            11 @ 11:24,
            "LED_GPIO" @ 11:13,
            (),
            11:9,
        ),
        FnCall(
            FnCallExpr {
                namespace: Some(
                    gpio,
                ),
                hashes: 9493536853530751511,
                args: [
                    Variable(LED_GPIO #1) @ 14:29,
                    StackSlot(0) @ 14:39,
                    StackSlot(1) @ 14:42,
                ],
                constants: [
                    0,
                    0,
                ],
                name: "enable_output",
                capture: false,
            },
            14:15,
        ),
        For(
            FnCall {
                name: "range",
                hash: 9199229206842614284,
                args: [
                    StackSlot(0) @ 17:24,
                    StackSlot(1) @ 17:27,
                ],
                constants: [
                    0,
                    10,
                ],
            } @ 17:18,
            (
                "i" @ 17:13,
                None,
                Block[
                    FnCall(
                        FnCallExpr {
                            namespace: Some(
                                gpio,
                            ),
                            hashes: 2692957339643980731,
                            args: [
                                Variable(LED_GPIO #2) @ 21:17,
                                FnCall {
                                    name: "%",
                                    hash: 9176748626153535436 (native only),
                                    args: [
                                        Variable(i #1) @ 22:17,
                                        StackSlot(0) @ 22:21,
                                    ],
                                    constants: [
                                        2,
                                    ],
                                } @ 22:19,
                            ],
                            constants: [],
                            name: "output_set",
                            capture: false,
                        },
                        20:19,
                    ),
                    FnCall(
                        FnCallExpr {
                            namespace: None,
                            hashes: 3513549430490172659,
                            args: [
                                StackSlot(0) @ 26:24,
                            ],
                            constants: [
                                1000,
                            ],
                            name: "time_delay",
                            capture: false,
                        },
                        26:13,
                    ),
                ] @ 17:31,
            ),
            17:9,
        ),
        Var(
            40 @ 30:17,
            "a" @ 30:13,
            (),
            30:9,
        ),
        Var(
            2 @ 31:17,
            "b" @ 31:13,
            (),
            31:9,
        ),
        FnCall(
            FnCallExpr {
                namespace: None,
                hashes: 7915633995129343590 (native only),
                args: [
                    Variable(a #2) @ 32:9,
                    Variable(b #1) @ 32:13,
                ],
                constants: [],
                name: "+",
                capture: false,
            },
            32:11,
        ),
    ],
    functions: Module,
    resolver: None,
}
```

And we transcode each AST Node to uLisp...

```lisp
( let* () 
  ( loop 
    ( let* (( a 1 )) 
      ( print a )
      ( if ( eq a 1 ) 
        ( return )
      )
    )
  )
  ( let* (( LED_GPIO 11 )) 
    ( bl_gpio_enable_output LED_GPIO 0 0 )
    ( dotimes (i 10) 
      ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
      ( time_delay 1000 )
    )
    ( let* (( a 40 )) 
      ( let* (( b 2 )) 
        ( + a b )
      )
    )
  )
)
```

[See the complete log](src/lib.rs#L171-L709)
