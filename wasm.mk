# Build uLisp for WebAssembly: wasm/ulisp.html, ulisp.js, ulisp.wasm

###############################################################################
# Typical Compile to WebAssembly with emscripten
# emcc hello.c -s WASM=1 -o hello.html

# WebAssembly C and C++ Source Files
WASM_CSRCS := src/ulisp.c wasm/wasm.c

# Build uLisp app: wasm/ulisp.html, ulisp.js, ulisp.wasm
TARGETS:= wasm/ulisp

DEPS   := 

# Use emscripten compiler
CC     := emcc
CPP    := em++

# Options for emscripten. We specify the C WebAssembly 
# functions to be exported.
CCFLAGS := \
	-g \
	-I include \
	-s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_setup_ulisp', '_execute_ulisp', '_clear_simulation_events', '_get_simulation_events' ]" \
	-s "EXTRA_EXPORTED_RUNTIME_METHODS=[ 'cwrap', 'allocate', 'intArrayFromString', 'UTF8ToString' ]"

LDFLAGS := 

MAINS  := $(addsuffix .o, $(TARGETS) )
OBJ    := \
	$(MAINS) \
	$(CSRCS:.c=.o) \
	$(WASM_CSRCS:.c=.o)

.PHONY: all clean

all: $(TARGETS)

clean:
	rm *.o || true
	rm wasm/*.o || true
	rm wasm/*.wasm || true
	rm wasm/*.js || true
	rm wasm/*.txt || true
	rm -r $(HOME)/.emscripten_cache || true

$(OBJ): %.o : %.c $(DEPS)
	$(CC) -c -o $@ $< $(CCFLAGS)

# TODO: Build C++ files with em++
# $(OBJ): %.o : %.cpp $(DEPS)
#	$(CPP) -c -o $@ $< $(CCFLAGS)

$(TARGETS): % : $(filter-out $(MAINS), $(OBJ)) %.o
	$(CC) -o $@.html \
	-Wl,--start-group \
	$(LIBS) \
	$^ \
	-Wl,--end-group \
	$(CCFLAGS) \
	$(LDFLAGS)
	cp wasm/ulisp.js   docs
	cp wasm/ulisp.wasm docs
