# Minimal C program

Just a bare metal C program that makes use of the Newlib C runtime library.

## Quickstart

1. Build and install [Headsail-Newlib](https://github.com/andstepan/headsail-newlib) on a directory of your choice and paste the include path and library path the `INC_PATH` and `LIB_PATH` of the CMakeLists.txt, accordingly.
2. Make sure you've got `riscv64-unknown-elf-gcc` available on $PATH. 
3. Then `just run`. Use `just clean-build` to reconfigure from scratch (useful for testing & debugging the build process).
See Justfile for what goes on behind the scenes.

## Build Configuration Information

This program is based on the Headsail Newlib port, which is in itself based on Newlib 4.4.0. The crt0.S is pulled 
from the ported Newlib library, but the user should provide the linker script. The SDRAM linker script (`linker_sdram.lds`) should work just fine with Newlib, with regards to dynamic memory allocation. Additionally, 
in order for the program to link, `cmodel=medany` must be used in order to use the medany code model. All these 
issues are taken care of by the cmake file. The user should just build and install Newlib (and the `riscv64-unknown-elf-gcc` if not already available) and then correctly set the `INC_PATH` and `LIB_PATH` paths.
