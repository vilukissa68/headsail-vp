# Headsail BSP Rust cbindgen

Compile c program with rust bsp

## Compile
The Cmake build uses CC environment variable for finding the correct compiler. Before building set CC.
``` sh
export CC=/path/to/riscv-gcc
```

Build with cmake
``` sh
mkdir build
cd build
cmake .. 
make
```

