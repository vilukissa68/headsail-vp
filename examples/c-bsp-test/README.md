# Headsail BSP Rust cbindgen

Compile c program with rust bsp

## Compile bsp
First we need to compile the headsail-bsp. Bsp is written in rust so we use a FFI (foreign function interface) for exposing the bsp to C.
``` sh
cd headsail-vp/scripts
./run_on_hpc.sh ../examples/c-bsp-test/build/main
```

## Compile binary
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

### Enabling Renode UART

### Running on VP
The binary can be run on headsail's virtual prototype in Renode. Install Renode according to instructions in [Renode documentations](https://renode.readthedocs.io/en/latest/introduction/installing.html). After succesfully installing renode nagivate to the headsails-VP scripts directory and run the binary in Renode with the run_on_hpc.sh script.

``` sh
cd headsail-vp/scripts
./run_on_hpc.sh ../examples/c-bsp-test/build/main
```
