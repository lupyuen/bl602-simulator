//  WebAssembly Interface for uLisp
#ifndef ULISP_WASM_H
#define ULISP_WASM_H
#include <stdint.h>

int bl_gpio_enable_input(uint8_t pin, uint8_t pullup, uint8_t pulldown);
int bl_gpio_enable_output(uint8_t pin, uint8_t pullup, uint8_t pulldown);
int bl_gpio_output_set(uint8_t pin, uint8_t value);
void time_delay(uint32_t ticks);

//  1 tick is 1 millisecond
static inline uint32_t time_ms_to_ticks32(uint32_t millisec) { return millisec; }

#endif  //  ULISP_WASM_H
