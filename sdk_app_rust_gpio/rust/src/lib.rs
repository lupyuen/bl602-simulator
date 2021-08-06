//!  Blink the LED connected to a GPIO Pin
//// TODO: #![no_std]  //  Use the Rust Core Library instead of the Rust Standard Library, which is not compatible with embedded systems
#![feature(libc)]  ////  TODO: Allow C Standard Library, which will be mapped by emscripten to JavaScript

//  Import Libraries
use bl602_sdk::{       //  Rust Wrapper for BL602 IoT SDK
    gpio,              //  GPIO HAL
    puts,              //  Console Output
    time_delay,        //  NimBLE Time Functions
    time_ms_to_ticks32,
};

/// This function will be called by the BL602 command-line interface
#[no_mangle]              //  Don't mangle the function name
extern "C" fn rust_main(  //  Declare `extern "C"` because it will be called by BL602 firmware
    _result: *mut u8,        //  Result to be returned to command-line interface (char *)
    _len:  i32,              //  Size of result buffer (int)
    _argc: i32,              //  Number of command line args (int)
    _argv: *const *const u8  //  Array of command line args (char **)
) {
    //  Show a message on the serial console
    puts("Hello from Rust!");

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

    //  Return to the BL602 command-line interface
}

/// This function is called on panic, like an assertion failure
#[panic_handler]
#[cfg(not(target_arch = "wasm32"))]  ////  TODO
fn panic(_info: &core::panic::PanicInfo) -> ! {  //  `!` means that panic handler will never return
    //  TODO: Implement the complete panic handler like this:
    //  https://github.com/lupyuen/pinetime-rust-mynewt/blob/master/rust/app/src/lib.rs#L115-L146

    //  For now we display a message
    puts("TODO: Rust panic"); 

	//  Loop forever, do not pass go, do not collect $200
    loop {}
}

/* Output Log
*/