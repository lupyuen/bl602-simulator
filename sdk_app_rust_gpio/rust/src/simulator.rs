//!  BL602 Simulator for WebAssembly

//  Import the serde crate for JSON Serialization
use serde::{Serialize, Deserialize};

/// Event to be simulated by the BL602 Simulator
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum SimulationEvent {
    /// GPIO Set Output:
    /// { "gpio_output_set": { "pin": 11, "value": 1 }
    gpio_output_set {
        pin:   u8,
        value: u8,
    },
    /// Time Delay:
    /// { "time_delay": { "ticks": 1000 } }
    time_delay {
        ticks: u32,
    },
}

/// Vector of Simulation Events (i.e. event array)
static mut SIMULATION_EVENTS: Vec<SimulationEvent> = Vec::new();

/// String Buffer that returns the JSON Stream of Simulation Events:
/// [ { "gpio_output_set": { "pin": 11, "value": 1 } }, 
///   { "time_delay": { "ticks": 1000 } }, 
///   ... 
/// ]
static mut EVENT_BUFFER: [u8; 1024] = [0; 1024];

/// Clear the JSON Stream of Simulation Events
#[no_mangle]  //  Don't mangle the function name
extern "C" fn clear_simulation_events() {
    unsafe {
        SIMULATION_EVENTS.clear();
    }
}

/// Return the JSON Stream of Simulation Events
#[no_mangle]  //  Don't mangle the function name
extern "C" fn get_simulation_events() -> *const u8 {
    //  Convert vector of events to a JSON string
    let mut serialized = unsafe {
        serde_json::to_string(&SIMULATION_EVENTS)
    }.unwrap();

    //  Print the serialized JSON events
    println!("get_simulation_events: {}", serialized);

    //  Result:
    //  [{"gpio_output_set":{"pin":11,"value":0}},
    //   {"time_delay":{"ticks":1000}}]

    //  Terminate the JSON string with null, since we will be returning to C
    serialized.push('\0');

    //  Check that JSON string fits into the Event Buffer
    assert!(serialized.len() <= unsafe { EVENT_BUFFER.len() });

    //  Copy the JSON string to the Event Buffer
    unsafe {                            //  Unsafe because we are copying raw memory
        std::ptr::copy(                 //  Copy the memory...
            serialized.as_ptr(),        //  From Source (JSON String)
            EVENT_BUFFER.as_mut_ptr(),  //  To Destination (mutable pointer to Event Buffer)
            serialized.len()            //  Number of Items (each item is 1 byte)
        );    
    }
      
    //  Return the Event Buffer
    unsafe {
        EVENT_BUFFER.as_ptr()
    }
}

/// Configure a GPIO Pin for Input Mode. See `bl_gpio_enable_input` in "Enable GPIO" <https://lupyuen.github.io/articles/led#enable-gpio>
#[no_mangle]  //  Don't mangle the function name
extern "C" fn bl_gpio_enable_input(_pin: u8, _pullup: u8, _pulldown: u8)
-> c_int {
    //  TODO
    0  //  Return OK
}

/// Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in "Enable GPIO" <https://lupyuen.github.io/articles/led#enable-gpio>
#[no_mangle]  //  Don't mangle the function name
extern "C" fn bl_gpio_enable_output(_pin: u8, _pullup: u8, _pulldown: u8)
-> c_int {
    //  TODO
    0  //  Return OK
}

/// Set the output value of a GPIO Pin. See `bl_gpio_output_set` in "Read and Write GPIO" <https://lupyuen.github.io/articles/led#read-and-write-gpio>
#[no_mangle]  //  Don't mangle the function name
extern "C" fn bl_gpio_output_set(pin: u8, value: u8)
-> c_int {
    // Add a GPIO Set Output event
    let ev = SimulationEvent::gpio_output_set { 
        pin,
        value,
    };
    unsafe {
        SIMULATION_EVENTS.push(ev);
    }
    0  //  Return OK
}

/// Sleep for the specified number of system ticks.  (From NimBLE Porting Layer)
/// `void ble_npl_time_delay(ble_npl_time_t ticks)`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn ble_npl_time_delay(
    ticks: u32  //  Number of ticks to sleep
) {
    // Add a Time Delay event
    let ev = SimulationEvent::time_delay { 
        ticks,
    };
    unsafe {
        SIMULATION_EVENTS.push(ev);    
    }
}

/// Convert milliseconds to system ticks.  (From NimBLE Porting Layer)
/// 1 tick is 1 millisecond.
/// `ble_npl_time_t ble_npl_time_ms_to_ticks32(uint32_t ms)`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn ble_npl_time_ms_to_ticks32(
    ms: u32  //  Number of milliseconds
) -> u32 {   //  Returns the number of ticks (uint32_t)    
    ms       //  1 tick is 1 millisecond
}

/// Return type for BL602 IoT SDK
#[allow(non_camel_case_types)]
type c_int = i32;
