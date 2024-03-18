# Headsail - Virtual Platform

## Software testing requirements

1. Rust <https://rustup.rs/>
2. Codegen backend for target cores
    * `rustup target add riscv64imac-unknown-none-elf`
    * `rustup target add riscv32imc-unknown-none-elf`

### Run UART example

```sh
cd examples/hello-hpc
cargo run --example uart0
```

### Run DLA example

```sh
cd examples/hello-dla
```

Set renode installation path to RENODE_PATH variable in renode.sh

```sh
cargo run --example dla
```

### Run Robot Tests

```sh
renode-test scripts/robot/hello_dla.robot
```
