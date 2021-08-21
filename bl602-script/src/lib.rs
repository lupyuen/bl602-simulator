//! BL602 Rust Scripting Library

#![feature(libc)]  //  Allow C Standard Library, which will be mapped by emscripten to JavaScript

extern crate alloc;
extern crate wee_alloc;

//  Use `wee_alloc` as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod scope;
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
    "#;

    //  Compile Rhai Script to an Abstract Syntax Tree
    let ast = engine.compile(script)
        .unwrap();
    println!("AST: {:#?}", ast);

    //  Transcode the Rhai Abstract Syntax Tree to uLisp
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
                        hashes: 67527529886503918,
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
                        hash: 17995251237036173671 (native only),
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
                hashes: 12987214658708294900,
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
                hash: 575929612303946337,
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
                            hashes: 11353779519759374485,
                            args: [
                                Variable(LED_GPIO #2) @ 21:17,
                                FnCall {
                                    name: "%",
                                    hash: 3751886575790804804 (native only),
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
                            hashes: 16488626815644268959,
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
                hashes: 749902770210084069 (native only),
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
begin: let* ()
Node: Stmt(
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
                    hashes: 67527529886503918,
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
                    hash: 17995251237036173671 (native only),
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
)
begin: loop
begin: let* (( a 1 ))
add:   ( print a )
begin: if ( eq a 1 )
add:   ( return )
add:   ( if ( eq a 1 ) 
  ( return )
)
add:   ( loop 
  ( let* (( a 1 )) 
    ( print a )
    ( if ( eq a 1 ) 
      ( return )
    )
  )
)
Node: Stmt(
    Var(
        11 @ 11:24,
        "LED_GPIO" @ 11:13,
        (),
        11:9,
    ),
)
begin: let* (( LED_GPIO 11 ))
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: Some(
                gpio,
            ),
            hashes: 12987214658708294900,
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
)
add:   ( bl_gpio_enable_output LED_GPIO 0 0 )
Node: Stmt(
    For(
        FnCall {
            name: "range",
            hash: 575929612303946337,
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
                        hashes: 11353779519759374485,
                        args: [
                            Variable(LED_GPIO #2) @ 21:17,
                            FnCall {
                                name: "%",
                                hash: 3751886575790804804 (native only),
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
                        hashes: 16488626815644268959,
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
)
begin: dotimes (i 10)
add:   ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
add:   ( time_delay 1000 )
add:   ( dotimes (i 10) 
  ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
  ( time_delay 1000 )
)
Node: Stmt(
    Var(
        40 @ 30:17,
        "a" @ 30:13,
        (),
        30:9,
    ),
)
begin: let* (( a 40 ))
Node: Stmt(
    Var(
        2 @ 31:17,
        "b" @ 31:13,
        (),
        31:9,
    ),
)
begin: let* (( b 2 ))
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: None,
            hashes: 749902770210084069 (native only),
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
)
add:   ( + a b )
Transcoded uLisp:
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