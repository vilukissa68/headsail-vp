cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

set(XLEN 64)
set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)
set(ARCH rv64imac)
set(ABI lp64)

project(headsail-tvm C ASM)

# Path to onnx model
set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/add.onnx)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
#find_package(RiscvGcc REQUIRED) # CMAKE_C_COMPILER, CMAKE_ASM_COMPILER, CMAKE_LINKER
#find_package(Bsp REQUIRED) # BSP_INCLUDE_DIR, BSP_LIBRARIES

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=${ARCH} -mabi=${ABI} -Fuse-ld=riscv-none-elf-ld -nostdlib -T${CRT0}/linker_sdram.lds -mcmodel=medany")

# Add the source files
set(SOURCE_FILES
    src/main.c
    src/bundle_static.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib0.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib1.c
    #${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/devc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/graph_c.json.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/params_c.bin.c
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

# Link newlib
set(NEWLIB /Users/vainogranat/work/newlib-build/build/riscv64-unknown-elf/lib/${ARCH}/${ABI})
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/crt0.o)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lgloss -lgcc)

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/crt_config)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/libtvm_runtime.a)
set(TVM_NUM_THREADS=1)

# target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib0.o)
# target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib1.o)
# target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/devc.o)


# Add BSP header
#target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${BSP_INCLUDE_DIR})

# Link bsp static lib
#target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${BSP_LIBRARIES})

# Build codegen library
execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/build_module.py -m ${ONNX_MODEL} -o ./model_c)
