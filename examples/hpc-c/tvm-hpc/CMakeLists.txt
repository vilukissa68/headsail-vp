cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(headsail-tvm C)

# Path to onnx model
set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/model.onnx)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED) # CMAKE_C_COMPILER, CMAKE_ASM_COMPILER, CMAKE_LINKER
find_package(Bsp REQUIRED) # BSP_INCLUDE_DIR, BSP_LIBRARIES

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Os -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${CRT0}/linker_sdram.lds")

# Add the source files
set(SOURCE_FILES
    src/main.c
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

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES})

# Add C-runtime 0 header
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${CRT0})

# Add BSP header
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${BSP_INCLUDE_DIR})

# Link bsp static lib
target_link_libraries(${CMAKE_PROJECT_NAME} ${BSP_LIBRARIES})

# Include utvm runtime
target_include_directories(${CMAKE_PROJECT_NAME}
    PUBLIC $ENV{TVM_HOME}/build/standalone_crt/include
)

# Add crt config
target_include_directories(${CMAKE_PROJECT_NAME}
    PUBLIC $ENV{TVM_HOME}/build/crt_config
)

target_link_libraries(${CMAKE_PROJECT_NAME})

# Build codegen library
execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/generate_crt.py -m ${ONNX_MODEL} -o ./model_c)
