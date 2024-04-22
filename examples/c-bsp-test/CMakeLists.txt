cmake_minimum_required(VERSION 3.10)
project(RISCV_Compiler C)

## Add bsp
set(HEADSAIL_BSP_FFI ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi)
set(HEADSAIL_BSP_HPC_DIR  ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target/riscv64imac-unknown-none-elf/debug)

# Build bsp
add_custom_command(
    OUTPUT ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a
    COMMAND cd ${HEADSAIL_BSP_FFI} && just build-hpc
    WORKING_DIRECTORY ${HEADSAIL_BSP_FFI}
    COMMENT "Building bsp"
)

add_custom_target(build_hpc DEPENDS ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a)

# Find memory layout
execute_process(COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/find_linkx.sh ${HEADSAIL_BSP_FFI} riscv64
                OUTPUT_VARIABLE LINKX
                OUTPUT_STRIP_TRAILING_WHITESPACE)

execute_process(COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/find_mem.sh ${HEADSAIL_BSP_FFI}
                OUTPUT_VARIABLE MEM_HPC
                OUTPUT_STRIP_TRAILING_WHITESPACE)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${CMAKE_CURRENT_SOURCE_DIR}/linker.lds")

# Add the source file(s)
set(SOURCE_FILES main.c boot.h)

# Add the executable target
add_executable(main ${SOURCE_FILES})

# Add dependencies to ensure build_hpc is executed before building the main target
add_dependencies(main build_hpc)

# Add header
target_include_directories(main PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../headsail-bsp-ffi/target)

# Link bsp static lib
link_directories(${HEADSAIL_BSP_HPC_DIR})
target_link_libraries(main ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a)

message(STATUS ${HEADSAIL_BSP_HPC_DIR})
message(STATUS ${MEM_HPC})
message(STATUS ${LINKX})
