//  Export the inline functions for NimBLE Porting Layer to Rust
//  TODO: Move this to nimble-porting-layer library

//  Include FreeRTOS before NPL, so that FreeRTOS will be inlined
#include "FreeRTOS.h"

//  Disable static inline so:
//    static inline void ble_npl_time_delay(ble_npl_time_t ticks) { ... }
//  Becomes:
//    void ble_npl_time_delay(ble_npl_time_t ticks) { ... }
#define static
#define inline

//  Define the functions like:
//    void ble_npl_time_delay(ble_npl_time_t ticks) { ... }
#include "nimble_npl.h"
