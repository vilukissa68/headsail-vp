# Headsail - Virtual Platform

## Software testing requirements

1. Renode 1.14, the high-level hardware simulator <https://github.com/renode/renode>
2. Rust <https://rustup.rs/>
3. Codegen backend for target cores
    * HPC
        * `rustup target add riscv64imac-unknown-none-elf`
    * SysCtrl (VP)
        * `rustup target add riscv32im-unknown-none-elf`
    * SysCtrl (ASIC)
        * `rustup target add riscv32imc-unknown-none-elf`
4. (optional) Install `just`, the command runner <https://github.com/casey/just>

### Run UART example (VP)

```sh
cd examples/headsail-bsp
cargo run --example uart0 -Fvp -Fhpc-rt -Fpanic-apb-uart0 --target riscv64imac-unknown-none-elf
```

or if you're a `just` user:

```sh
just run uart0
```

### Run DLA example

```sh
cd examples/hpc/hello-dla
cargo run --example dla
```

## Run Robot Tests

You'll need to have [built the binaries](#run-dla-example) prior to running Robot Tests.

```sh
renode-test scripts/robot/hello_dla.robot
```

## Run an arbitrary ELF on Headsail

```sh
./scripts/run_on_hpc.sh $bin
./scripts/run_on_sysctrl.sh $bin
```
