//!  BL602 Simulator for WebAssembly

/// Convert milliseconds to system ticks.  (From NimBLE Porting Layer)
/// 1 tick is 1 millisecond.
/// `ble_npl_time_t ble_npl_time_ms_to_ticks32(uint32_t ms)`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn ble_npl_time_ms_to_ticks32(
    ms: u32  //  Number of milliseconds
) -> u32 {   //  Returns the number of ticks (uint32_t)    
    ms       //  1 tick is 1 millisecond
}

/*
/// Sleep for the specified number of system ticks.  (From NimBLE Porting Layer)
/// `void ble_npl_time_delay(ble_npl_time_t ticks)`
#[no_mangle]  //  Don't mangle the function name
extern "C" fn ble_npl_time_delay(
    ticks: u32  //  Number of ticks to sleep
) {
}
*/