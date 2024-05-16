set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
set(CMAKE_ASM_COMPILER $ENV{CC}/../riscv${XLEN}-unknown-elf-as)
set(CMAKE_LINKER $ENV{CC}/../riscv${XLEN}-unknown-elf-ld)
