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
    INT
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

    //  Register our functions with Rhai
    engine.register_fn("gpio_enable_output", gpio_enable_output);
    engine.register_fn("gpio_output_set",    gpio_output_set);

    //  Evaluate a Rhai Script
    let result = engine.eval::<INT>(
        //  Rhai Script to be evaluated
        r#" 
            gpio_enable_output(11, 0, 0);
            gpio_output_set(11, 0);
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

/// Rhai Stub for Enable GPIO Output
/// TODO: Modified parameters from u8 to i32
pub fn gpio_enable_output(pin: i32, pullup: i32, pulldown: i32) {
    //  Format the output and display it
    let mut buf = String::new();
    write!(buf, "gpio_enable_output: pin={}, pullup={}, pulldown={}\r\n", pin, pullup, pulldown)
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
fn gpio_output_set(pin: i32, value: i32) {
    //  Format the output and display it
    let mut buf = String::new();
    write!(buf, "gpio_output_set: pin={}, value={}\r\n", pin, value)
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

//  TODO: Fix `emscripten_get_now`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn _emscripten_get_now() -> f64 { 0.0 }

/// Return type for BL602 IoT SDK
#[allow(non_camel_case_types)]
type c_int = i32;
