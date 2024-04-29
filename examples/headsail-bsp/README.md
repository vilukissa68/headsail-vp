# Headsail BSP

Minimal BSP for testing the virtual prototype.

## Compile all examples

```sh
# HPC
RUSTFLAGS="-C link-arg=-Tmem_hpc.x -C link-arg=-Tlink.x" cargo build --examples -Fpanic-uart -Fhpc-rt -Falloc --target riscv64imac-unknown-none-elf

# SysCtrl
RUSTFLAGS="-C link-arg=-Tmem_sysctrl.x -C link-arg=-Tlink.x" cargo build --examples -Fpanic-uart -Fsysctrl-rt --target riscv32imc-unknown-none-elf
```

## Running examples

Make sure [examples are built](#compile-all-examples).

```sh
# Run on HPC
TEST_NAME=uart0 && renode --console -e "set bin @$(find target -name $TEST_NAME | grep riscv64); include @../../scripts/2_run_hpc.resc"

# Run on SysCtrl
TEST_NAME=uart0 && renode --console -e "set bin @$(find target -name $TEST_NAME | grep riscv32); include @../../scripts/2_run_sysctrl.resc"
```
