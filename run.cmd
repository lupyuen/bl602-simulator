::  Compile the library on Windows

..\xpack-riscv-none-embed-gcc\bin\riscv-none-embed-gcc -c -D__EMSCRIPTEN__ -I include src\*.c
..\xpack-riscv-none-embed-gcc\bin\riscv-none-embed-gcc -c -D__EMSCRIPTEN__ wasm\*.c
del *.o
