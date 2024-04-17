# Headsail BSP (FFI)

## Usage

- Include the generated header
  - `-include headsail_bsp.h`
- Link the generated staticlib
  - `-llibheadsail_bsp_ffi`

## Using the BSP as a runtime

This crate can be used as a runtime with the following steps:

1. Add the runtime feature
    - For HPC, compile with `-Fhpc-rt` (or `just build-hpc`)
    - SysCtrl by compiling with `-Fsysctrl-rt` (or `just build-sysctrl`)

2. Find the link scripts provided by `headsail-bsp`:
    - `find -name mem_hpc.x`
    - `find -name link.x`

3. Use the provided link strategy:
    - `CFLAGS+="-Tmem_hpc.x -Tlink.x"`

4. Make sure your entry point is named `cmain`, e.g., `int cmain() {}`
