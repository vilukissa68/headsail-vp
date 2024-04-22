# Headsail BSP Rust cbindgen

Compile c program with rust bsp

## Compile
The cmake build depends on the RISCV toolchain. Toolchain should be specified in the RISC-V-Toolchain.cmake file.
``` sh
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

set(CMAKE_C_COMPILER <path-to-riscv64-unknown-elf-gcc>)
set(CMAKE_ASM_COMPILER <path-to-riscv64-unknown-elf-as>)
set(CMAKE_LINLKER <path-to-riscv64-unknown-elf-ld>)
```

Build with cmake
``` sh
mkdir build
cd build
cmake -DCMAKE_TOOLCHAIN_FILE=../RISC-V-Toolchain.cmake
make
```

