//!  BL602 Rust Scripting Library

#![feature(libc)]  //  Allow C Standard Library, which will be mapped by emscripten to JavaScript

extern crate alloc;
extern crate wee_alloc;

//  Use `wee_alloc` as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//  Import Libraries
use core::{            //  Rust Core Library
    fmt::Write,        //  String Formatting    
};
use rhai::{            //  Rhai Scripting Engine
    Engine, 
    INT,
    plugin::*,
};

use bl602_sdk::{       //  Rust Wrapper for BL602 IoT SDK
    puts,              //  Console Output
    String,            //  Strings (limited to 64 chars)
};

/// This function will be called by WebAssembly to run a script
#[no_mangle]                 //  Don't mangle the function name
extern "C" fn rust_script(   //  Declare `extern "C"` because it will be called by BL602 firmware
    _result: *mut u8,        //  Result to be returned to command-line interface (char *)
    _len:  i32,              //  Size of result buffer (int)
    _argc: i32,              //  Number of command line args (int)
    _argv: *const *const u8  //  Array of command line args (char **)
) {
    //  Show a message on the serial console
    puts("Hello from Rust Script!\r\n");

    //  Init the Rhai script engine
    let mut engine = Engine::new();
    puts("Created script engine\r\n");

    //  Create a Rhai module from the plugin module
    let module = exported_module!(gpio);

    //  Register our module as a Static Module
    engine.register_static_module("gpio", module.into());

    //  Register our functions with Rhai
    engine.register_fn("time_delay", time_delay);

    //  Evaluate a Rhai Script
    let result = engine.eval::<INT>(
        //  Rhai Script to be evaluated
        r#" 
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
        "#
    ).unwrap() as isize;
    puts("Eval OK\r\n");

    //  Format the output and display it
    let mut buf = String::new();
    write!(buf, "Result of Rhai Script: {}\r\n", result)
        .expect("buf overflow");
    puts(&buf);
}

/// GPIO Module will be exported to Rhai as a Static Module
#[export_module]
mod gpio {
    /// Rhai Stub for Enable GPIO Output
    /// TODO: Modified parameters from u8 to i32
    pub fn enable_output(pin: i32, pullup: i32, pulldown: i32) {
        //  Format the output and display it
        let mut buf = String::new();
        write!(buf, "gpio::enable_output: pin={}, pullup={}, pulldown={}\r\n", pin, pullup, pulldown)
            .expect("buf overflow");
        puts(&buf);

        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
            -> c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let _res =
                bl_gpio_enable_output(pin as u8, pullup as u8,
                                        pulldown as u8);
            "----------Result----------";
            //  TODO: Throw exception in case of error
            //  match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }

    /// Rhai Stub for Set GPIO Output
    /// TODO: Modified parameters from u8 to i32
    pub fn output_set(pin: i32, value: i32) {
        //  Format the output and display it
        let mut buf = String::new();
        write!(buf, "gpio::output_set: pin={}, value={}\r\n", pin, value)
            .expect("buf overflow");
        puts(&buf);

        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_output_set(pin: u8, value: u8)
            -> c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let _res = bl_gpio_output_set(pin as u8, value as u8);
            "----------Result----------";
            //  TODO: Throw exception in case of error
            //  match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
}

/// Rhai Stub for Time Delay
/// TODO: Modified parameter from u32 to i32
pub fn time_delay(
    ticks: i32  //  Number of ticks to sleep
) {
    //  Format the output and display it
    let mut buf = String::new();
    write!(buf, "time_delay: {}\r\n", ticks)
        .expect("buf overflow");
    puts(&buf);

    extern "C" {  //  Import C Function
        /// Sleep for the specified number of system ticks (from NimBLE Porting Layer)
        fn ble_npl_time_delay(ticks: u32);
    }

    //  Call the C function
    unsafe {  //  Flag this code as unsafe because we're calling a C function
        ble_npl_time_delay(ticks as u32);
    }
}

//  TODO: Fix `emscripten_get_now`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn _emscripten_get_now() -> f64 { 0.0 }

/// Return type for BL602 IoT SDK
#[allow(non_camel_case_types)]
type c_int = i32;
