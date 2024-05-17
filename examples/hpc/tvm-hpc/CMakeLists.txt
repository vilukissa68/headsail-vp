cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)
project(headsail-tvm C)

# Path to onnx model
set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/../../../../tvm-headsail/torch/add.onnx)

## Add bsp
set(HEADSAIL_BSP_FFI ${CMAKE_CURRENT_SOURCE_DIR}/../../headsail-bsp-ffi)
set(HEADSAIL_BSP_HPC_DIR  ${HEADSAIL_BSP_FFI}/target/riscv64imac-unknown-none-elf/debug)

# Build bsp
add_custom_command(
    OUTPUT ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a
    COMMAND cd ${HEADSAIL_BSP_FFI} && just build-hpc
    WORKING_DIRECTORY ${HEADSAIL_BSP_FFI}
    COMMENT "Building bsp"
)

# Bin depends on bsp-ffi
add_custom_target(build_hpc DEPENDS ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${CMAKE_CURRENT_SOURCE_DIR}/linker.lds")

# Flags to reduce bin size
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Os --function-sections -fdata-sections")
set(LINK_FLAGS "${LINK_FLAGS} -Wl --gc-sections")

# Add the source files
set(SOURCE_FILES
    src/main.c
    src/boot.h
    src/bundle_static.c
    model_c/lib0.c
    model_c/lib1.c
    model_c/devc.c
    model_c/graph_c.json.c
    model_c/params_c.bin.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/crt_runtime_api.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/graph_executor/graph_executor.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/memory/page_allocator.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/packed_func.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/func_registry.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/graph_executor/load_json.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/crt_runtime_api.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/ndarray.c
    $ENV{TVM_HOME}/build/standalone_crt/src/runtime/crt/common/crt_backend_api.c
)

# Define executable
add_executable(utvmcrt.elf ${SOURCE_FILES})

# Add dependencies to ensure build_hpc is executed before building the main target
add_dependencies(utvmcrt.elf build_hpc)

# Add bsp header
target_include_directories(utvmcrt.elf PRIVATE ${HEADSAIL_BSP_FFI}/target)

# Link bsp static lib
link_directories(${HEADSAIL_BSP_HPC_DIR})
target_link_libraries(utvmcrt.elf ${HEADSAIL_BSP_HPC_DIR}/libheadsail_bsp_ffi.a)

# Include utvm runtime
target_include_directories(utvmcrt.elf
    PUBLIC $ENV{TVM_HOME}/build/standalone_crt/include
)

# Add crt config
target_include_directories(utvmcrt.elf
    PUBLIC $ENV{TVM_HOME}/build/crt_config
)

target_link_libraries(utvmcrt.elf)

# Build codegen library
execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/build_module.py -m ${ONNX_MODEL} -o ./model_c)
