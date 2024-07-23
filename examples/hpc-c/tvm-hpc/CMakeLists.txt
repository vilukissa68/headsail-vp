cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

set(XLEN 64)
set(CMAKE_C_COMPILER $ENV{CC})
set(CMAKE_CXX_COMPILER $ENV{CXX})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
set(LIBGCC "/opt/homebrew/opt/riscv-gnu-toolchain/lib/gcc/riscv64-unknown-elf/13.2.0/rv64imac/lp64/libgcc.a")

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)
set(ARCH rv64imac)
set(ABI lp64)

project(headsail-tvm C ASM)

# Choose target model
option(USE_MOBILENET "Use mobilenet" OFF)
option(USE_CONV2DBASIC "Use conv2dbasic" OFF)
option(USE_ADDER "Use adder" OFF)

if (USE_MOBILENET)
    list(APPEND SOURCE_FILES src/mobile_net.c)
    set(MODEL_NAME mobilenet)
    set(STIMULUS ../mobile_net_input.npy)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/mobilenet.onnx)
elseif (USE_CONV2DBASIC)
    list(APPEND SOURCE_FILES src/conv2dbasic.c)
    set(MODEL_NAME conv2dbasic)
    set(STIMULUS ../stimulus.npy)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/model.onnx)
elseif (USE_ADDER)
    list(APPEND SOURCE_FILES src/adder.c)
    set(MODEL_NAME add)
    set(STIMULUS ../mobile_net_input.npy)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/add.onnx)
else()
    message(FATAL_ERROR "No model selected")
endif()

# Build codegen library
execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/build_module.py -m ${MODEL_NAME} -p ${ONNX_MODEL} -o ./model_c -s ${STIMULUS} -t "float")

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=${ARCH} -mabi=${ABI} -Fuse-ld=riscv-none-elf-ld -nostdlib -T${CRT0}/linker_sdram.lds -mcmodel=medany")

# Gather all lib*.c files from a specific directory
file(GLOB LIB_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib*.c)
message(GLOB="${LIB_SOURCES}")

# Add the source files
set(SOURCE_FILES
    src/main.c
    src/bundle_static.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib0.o
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib1.o
    #${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/devc.o
    ${LIB_SOURCES}
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/graph_c.json.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/params_c.bin.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/stimulus.c
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
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIBGCC})
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lm -lgloss -lgcc)

# Include TVM headers
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include/dlpack)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include/tvm)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/crt_config)
