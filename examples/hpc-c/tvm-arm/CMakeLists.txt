cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)

set(CMAKE_C_COMPILER "/usr/bin/gcc")
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
project(arm-tvm C ASM)

# Path to onnx model
#set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/add.onnx)
set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/mobilenet.onnx)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0")

# Add the source files
set(SOURCE_FILES
    src/main.c
    src/bundle_static.c
    src/mobile_net.c
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib0.o
    ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/lib1.o
    #${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/devc.c
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

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/crt_config)
#target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/libtvm_runtime.a)
set(TVM_NUM_THREADS=1)

#execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/build_module.py -m mobilenet -p ${ONNX_MODEL} -o ./model_c -s ../mobile_net_input.npy -t "float")
