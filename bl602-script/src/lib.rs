//!  BL602 Rust Scripting Library

#![feature(libc)]  //  Allow C Standard Library, which will be mapped by emscripten to JavaScript

extern crate alloc;
extern crate wee_alloc;

//  Use `wee_alloc` as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod transcode;

//  Import Libraries
use rhai::{            //  Rhai Scripting Engine
    Engine, 
    plugin::*,
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
    println!("Hello from Rust Script!");

    //  Init the Rhai script engine
    let mut engine = Engine::new();
    println!("Created script engine");

    //  Create a Rhai module from the plugin module
    let module = exported_module!(gpio);

    //  Register our module as a Static Module
    engine.register_static_module("gpio", module.into());

    //  Register our functions with Rhai
    engine.register_fn("time_delay", time_delay);

    //  Rhai Script to be evaluated
    let script = r#" 
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
    "#;

    // Compile Rhai Script to an Abstract Syntax Tree
    let ast = engine.compile(script)
        .unwrap();
    println!("AST: {:#?}", ast);

    transcode::transcode(&ast);

    //  Evaluate the compiled Rhai Script
    let result: i32 = engine.eval_ast(&ast)
        .unwrap();
    println!("Eval OK");

    //  Alternatively: Evaluate a Rhai Script
    //  let result = engine.eval::<i32>(script).unwrap() as isize;

    //  Display the result
    println!("Result of Rhai Script: {}", result);
}

/// GPIO Module will be exported to Rhai as a Static Module
#[export_module]
mod gpio {
    /// Rhai Shim for Enable GPIO Output
    /// TODO: Modified parameters from u8 to i32
    pub fn enable_output(pin: i32, pullup: i32, pulldown: i32) {
        println!("gpio::enable_output: pin={}, pullup={}, pulldown={}", pin, pullup, pulldown);

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

    /// Rhai Shim for Set GPIO Output
    /// TODO: Modified parameters from u8 to i32
    pub fn output_set(pin: i32, value: i32) {
        println!("gpio::output_set: pin={}, value={}", pin, value);

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

/// Rhai Shim for Time Delay
/// TODO: Modified parameter from u32 to i32
pub fn time_delay(
    ticks: i32  //  Number of ticks to sleep
) {
    println!("time_delay: {}", ticks);
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

/* Output Log:

Execute: rust_script

Hello from Rust Script!
Created script engine
AST: AST {
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
gpio::enable_output: pin=11, pullup=0, pulldown=0
gpio::output_set: pin=11, value=0
time_delay: 1000
gpio::output_set: pin=11, value=1
time_delay: 1000
gpio::output_set: pin=11, value=0
time_delay: 1000
gpio::output_set: pin=11, value=1
time_delay: 1000
gpio::output_set: pin=11, value=0
time_delay: 1000
gpio::output_set: pin=11, value=1
time_delay: 1000
gpio::output_set: pin=11, value=0
time_delay: 1000
gpio::output_set: pin=11, value=1
time_delay: 1000
gpio::output_set: pin=11, value=0
time_delay: 1000
gpio::output_set: pin=11, value=1
time_delay: 1000
Eval OK
Result of Rhai Script: 42
get_simulation_events: [{"gpio_output_set":{"pin":11,"value":0}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":1}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":0}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":1}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":0}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":1}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":0}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":1}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":0}},{"time_delay":{"ticks":1000}},{"gpio_output_set":{"pin":11,"value":1}},{"time_delay":{"ticks":1000}}]
Events: [
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 0
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 1
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 0
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 1
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 0
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 1
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 0
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 1
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 0
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  },
  {
    "gpio_output_set": {
      "pin": 11,
      "value": 1
    }
  },
  {
    "time_delay": {
      "ticks": 1000
    }
  }
]
*/