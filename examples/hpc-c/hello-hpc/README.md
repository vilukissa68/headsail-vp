# Minimal C program

Just a bare metal C program.

## Quickstart

Make sure you've got a compatible `riscv64-unknown-elf-gcc` available on $PATH. Then `just run`. Use
`just clean-build` to reconfigure from scratch (useful for testing & debugging the build process).
See Justfile for what goes on behind the scenes.

## Compile & run

1. Set CC to point to RISC-V compiler
    * `source ../scripts/export_riscv.env`
2. Configure & build the project
    * `mkdir -p build && cd build`
    * `cmake ..`
    * `make`
3. Run the code on the virtual platform
    * `../../../../scripts/run_on_hpc.sh hello-hpc`
