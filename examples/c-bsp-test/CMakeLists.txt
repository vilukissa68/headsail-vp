cmake_minimum_required(VERSION 3.10)
project(RISCV_Compiler C)

## Add bsp
set(HEADSAIL_BSP_FFI ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi)
set(HEADSAIL_BSP_HPC_DIR  ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target/riscv64imac-unknown-none-elf/debug)

# Find memory layout
execute_process(COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/find_linkx.sh ${HEADSAIL_BSP_FFI} riscv64
                OUTPUT_VARIABLE LINKX
                OUTPUT_STRIP_TRAILING_WHITESPACE)

execute_process(COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/find_mem.sh ${HEADSAIL_BSP_FFI}
                OUTPUT_VARIABLE MEM_HPC
                OUTPUT_STRIP_TRAILING_WHITESPACE)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -v -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${MEM_HPC} -T${LINKX}")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(main ${SOURCE_FILES})

# Add header
target_include_directories(main PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target)

# Link bsp static lib
link_directories(${HEADSAIL_BSP_HPC_DIR})
target_link_libraries(main ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a)

message(STATUS ${HEADSAIL_BSP_HPC_DIR})
message(STATUS ${MEM_HPC})
message(STATUS ${LINKX})
