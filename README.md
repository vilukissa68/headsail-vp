# Headsail - Virtual Platform

## Software testing requirements

1. Rust <https://rustup.rs/>
2. Codegen backend for target cores
    * `rustup target add riscv64imac-unknown-none-elf`
    * `rustup target add riscv32im-unknown-none-elf`

### Run UART example

```sh
cd examples/headsail-bsp
cargo run --example uart0 -Fhpc-rt -Fpanic-apb-uart0 --target riscv64imac-unknown-none-elf
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
