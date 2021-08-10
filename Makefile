###############################################################################
# Compile Rust and C to WebAssembly with Emscripten

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

# Options for Emscripten. We specify the WebAssembly Functions to be exported.
# TODO: Change `_rust_main` to the Rust command names.
CCFLAGS := \
	-g \
	-s WASM=1 \
	-s DISABLE_EXCEPTION_CATCHING=0 \
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

# Compile C files with Emscripten
$(OBJ): %.o : %.c $(DEPS)
	$(CC) -c -o $@ $< $(CCFLAGS)

# Build the Rust Firmware and Rust Simulator Library and link with Emscripten
# TODO: Copy the WebAssembly outputs to docs folder manually in Windows
$(TARGETS): % : $(filter-out $(MAINS), $(OBJ)) %.o
	# Build the Rust Firmware and Rust Simulator Library
	cargo build --target wasm32-unknown-emscripten

	# Link the Rust Firmware and Rust Simulator Library with Emscripten
	$(CC) -o $@.html \
	$(LIBS) \
	$^ \
	$(CCFLAGS) \
	$(LDFLAGS)

	# Copy the WebAssembly outputs to the docs folder for GitHub Pages
	cp wasm/wasm.js   docs
	cp wasm/wasm.wasm docs
