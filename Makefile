###############################################################################
# Compile Rust and C to WebAssembly with emscripten

# WebAssembly C and C++ Source Files
WASM_CSRCS :=

# Build wasm app: wasm/wasm.html, wasm.js, wasm.wasm
TARGETS:= wasm/wasm

# Link with BL602 Rust Firmware compiled into WebAssembly
# and the BL602 Rust Simulator Library
LIBS   := \
	target/wasm32-unknown-emscripten/debug/libapp.a \
	target/wasm32-unknown-emscripten/debug/libbl602_simulator.a

# Use emscripten compiler
CC     := emcc
CPP    := em++

# Options for emscripten. We specify the C WebAssembly functions to be exported.
# TODO: Change "_rust_main" to the Rust command names.
CCFLAGS := \
	-g \
	-I include \
	-s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_rust_main', '_clear_simulation_events', '_get_simulation_events' ]" \
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
	cargo clean
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
	# Build the Rust Firmware and Rust Simulator Library
	cargo build --target wasm32-unknown-emscripten

	# Link the Rust Firmware and Rust Simulator Library with Emscripten
	$(CC) -o $@.html \
	-Wl,--start-group \
	$(LIBS) \
	$^ \
	-Wl,--end-group \
	$(CCFLAGS) \
	$(LDFLAGS)

	# Copy the WebAssembly outputs to the docs folder for GitHub Pages
	cp wasm/wasm.js   docs
	cp wasm/wasm.wasm docs
