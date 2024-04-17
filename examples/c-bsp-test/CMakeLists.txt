cmake_minimum_required(VERSION 3.10)
project(RISCV_Compiler C)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=rv64imac -mabi=lp64 -fno-builtin")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add bsp
set(HEADSAIL_BSP_DIR  ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target/riscv64imac-unknown-none-elf/debug)

# Add the executable target
add_executable(main ${SOURCE_FILES})

# Add header
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target)

# Link bsp
link_directories(${HEADSAIL_BSP_DIR})
target_link_libraries(main PUBLIC ${HEADSAIL_BSP_DIR}/libheadsail_bsp_ffi.a)
