# Headsail - Virtual Platform

## Directories

| Directory | Description                       |
| :-        | :-                                |
| .github   | CI specifications in GitHub Actions language |
| doc       | Auxiliary documentation           |
| examples  | C and Rust examples to run on device |
| scripts   | Renode & Shell scripts to run the virtual platform |
| vp        | The Renode virtual platform       |

## Software testing requirements

- Renode 1.14 (exactly 1.14), the high-level hardware simulator <https://github.com/renode/renode>
- For C-language validation tests:
  - [RISC-V GCC compiler toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain/)
  - [CMake 3.10](https://cmake.org/download/) or more recent
- For Rust-language validation tests & BSP:
  - Rust <https://rustup.rs/>, MSRV 1.81
  - Codegen backend for target cores
    - HPC
      - `rustup target add riscv64imac-unknown-none-elf`
    - SysCtrl (VP)
      - `rustup target add riscv32im-unknown-none-elf`
    - SysCtrl (ASIC)
      - `rustup target add riscv32imc-unknown-none-elf`
- For Renode & Rust command automation (optional):
  - Install `just`, the command runner <https://github.com/casey/just>, a make-like command runner
  that is used to automate some of the more complicated workflows.

### Run basic UART example (Renode VP, C)

Requires [GCC, CMake, Renode](#software-testing-requirements).

```sh
cd examples/hpc-c/hello-hpc
just run
```

If you're unable to use `just`, you might review the `Justfile` in that directory for the correct
set of CMake commands to run. File an issue if you run into a problem.

### Run basic UART example (Renode VP, Rust)

Requires [Rust, Renode](#software-testing-requirements).

```sh
cd examples/headsail-bsp
cargo run --example uart0 -Fvp -Fhpc-rt -Fpanic-apb-uart0 --target riscv64imac-unknown-none-elf
```

or if you're a `just` user:

```sh
just run uart0
```

### Run DLA example (Renode VP, Rust)

Requires [Rust, Renode](#software-testing-requirements).

```sh
cd examples/hpc/hello-dla
cargo run --example dla
```

## Run Robot Tests

You'll need to have [built the binaries](#run-dla-example-renode-vp-rust) prior to running Robot Tests.

```sh
renode-test scripts/robot/hello_dla.robot
```

## Run an arbitrary ELF on Headsail (VP)

Requires [Renode](#software-testing-requirements) and an ELF file. You could probably run the
OpenSBI firmware image with this process as well.

```sh
./scripts/run_on_hpc.sh $bin
./scripts/run_on_sysctrl.sh $bin
```

## Run an example on ASIC using both SysCtrl & HPC

I've set up a relatively convenient build using OpenOCD scripts & Justfiles.

1. To boot HPC, you will need to be connected on SysCtrl via OpenOCD.

    ```sh
    cd examples/headsail-bsp/openocd
    openocd -f sysctrl.cfg
    ```

2. On another terminal, use a `just` to run a bootloader to initialize HPC (remember to press 'c' to make the program run):

    ```sh
    cd examples/sysctrl/hello-sysctrl
    just asic init_hpc
    ```

3. Then disconnect OpenOCD from SysCtrl, and disconnect and re-connect the JTAG_TRST jumper cable.

4. Connect to HPC via OpenOCD (on the first terminal):

    ```sh
    openocd -f hpc.cfg
    ```

    Connection to HPC sometimes fails once before succeeding, so if at first you don't succeed, try again!

5. On a third terminal, run a program on HPC

    ```sh
    cd examples/hpc/hello-dla
    just asic apb_uart0
    ```

    GDB might report "load failed" but you might be able to just 'c' it. It seems to work most of the time ":D"

You can replace the final step with any GDB command of the general form:

`riscv64-unknown-elf-gdb -x connect-and-load.gdb your_binary_here`

GDB scripts such as `connect-and-load.gdb` may be sprinkled wherever they've been found
convenient.
