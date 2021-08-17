# BL602 / BL604 Rust Scripting Library

Based on Rhai Scripting Engine: https://rhai.rs/book

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
