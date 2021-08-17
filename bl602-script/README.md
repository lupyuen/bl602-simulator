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
