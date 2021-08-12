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

/// This function will be called by the BL602 command-line interface
#[no_mangle]              //  Don't mangle the function name
extern "C" fn rust_script(   //  Declare `extern "C"` because it will be called by BL602 firmware
    _result: *mut u8,        //  Result to be returned to command-line interface (char *)
    _len:  i32,              //  Size of result buffer (int)
    _argc: i32,              //  Number of command line args (int)
    _argv: *const *const u8  //  Array of command line args (char **)
) {
    //  Show a message on the serial console
    puts("Hello from Rust Script!\r\n");

    //  Notice that this is a _raw_ engine.
    //  To do anything useful, load a few packages from `rhai::packages`.
    let engine = Engine::new_raw();
    puts("a\r\n");

    //  Evaluate a simple Rhai Script: 40 + 2
    let result = engine.eval_expression::<INT>(
        //  Rhai Script to be evaluated
        "40 + 2"
        /*
        r#" 
            let a = 40; 
            let b = 2;
            a + b 
        "#
        */
    ).unwrap() as isize;
    puts("b\r\n");

    //  Format the output and display it
    let mut buf = String::new();
    write!(buf, "Result of Rhai Script: {}", result)
        .expect("buf overflow");
    puts(&buf);

    //  Return to the BL602 command-line interface
}
