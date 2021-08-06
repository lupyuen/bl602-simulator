//  WebAssembly Interface for uLisp
#include <setjmp.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "wasm.h"

/// JSON Stream of Simulation Events:
/// This uLisp script...
///   ( digitalwrite 11 :high )
///   ( delay 1000 )
/// Will generate this JSON Stream of Simulation Events...
/// [ { "gpio_output_set": { "pin": 11, "value": 1 } }, 
///   { "time_delay": { "ticks": 1000 } }, 
///   ... 
/// ]
static char events[1024] = "[]";

/// Clear the JSON Stream of Simulation Events
void clear_simulation_events(void) {
    strcpy(events, "[]");
}

/// Return the JSON Stream of Simulation Events
const char *get_simulation_events(void) {
    assert(events[0] == '[');
    assert(events[strlen(events) - 1] == ']');

    //  Erase the leading comma: "[,...]" becomes "[ ...]"
    if (events[1] == ',') { events[1] = ' '; }
    return events;
}

/// Preempt the uLisp task and allow background tasks to run.
/// Called by eval() and sp_loop() in src/ulisp.c
void yield_ulisp(void) {
    //  If uLisp is running a loop or recursion,
    //  the Simulation Events buffer may overflow.
    //  We stop before the buffer overflows.
    if (strlen(events) + 100 >= sizeof(events)) {  //  Assume 100 bytes of leeway
        //  Cancel the loop or recursion by jumping to loop_ulisp() in src/ulisp.c
        puts("Too many iterations, stopping the loop");
        extern jmp_buf exception;  //  Defined in src/ulisp.c
        longjmp(exception, 1);
    }
}

/// Add a GPIO event to enable input (0 for to disable pullup/pulldown, 1 to enable pullup/pulldown)
int bl_gpio_enable_input(uint8_t pin, uint8_t pullup, uint8_t pulldown) { 
    //  TODO
    return 0; 
}

/// Add a GPIO event to enable output (0 for to disable pullup/pulldown, 1 to enable pullup/pulldown)
int bl_gpio_enable_output(uint8_t pin, uint8_t pullup, uint8_t pulldown) { 
    //  TODO
    return 0; 
}

/// Add a GPIO event to set output (0 for low, 1 for high)
int bl_gpio_output_set(uint8_t pin, uint8_t value) {
    //  How many chars in the Simulation Events buffer to keep
    int keep = 
        strlen(events)  //  Keep the existing events
        - 1;            //  Skip the trailing "]"
    snprintf(
        events + keep,
        sizeof(events) - keep,
        ", { \"gpio_output_set\": { "
            "\"pin\": %d, "
            "\"value\": %d "
        "} } ]",
        pin,
        value
    );
    return 0; 
}

/// Add a delay event. 1 tick is 1 millisecond
void ble_npl_time_delay(uint32_t ticks) { 
    //  How many chars in the Simulation Events buffer to keep
    int keep = 
        strlen(events)  //  Keep the existing events
        - 1;            //  Skip the trailing "]"
    snprintf(
        events + keep,
        sizeof(events) - keep,
        ", { \"time_delay\": { "
            "\"ticks\": %d "
        "} } ]",
        ticks
    );
}
