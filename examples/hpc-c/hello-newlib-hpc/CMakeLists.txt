cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(hello-newlib-hpc C)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR})

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

# Find RiscV GCC
find_package(RiscvGcc REQUIRED)
# Find Newlib installation path
find_package(Newlib REQUIRED)

# Optimizations disabled and debug symbols enabled
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0")
# Architecture and ABI definition
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=rv64imac -mabi=lp64")
# Do not use the standard libraries provided by the compiler
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -nostdlib")
# Use this particular linker script
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -T${CRT0}/../crt0/linker_sdram.lds")
# Use the medany RISCV code model and the specified linker
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mcmodel=medany -Fuse-ld=riscv64-unknown-elf-ld")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES} ${LIB_PATH}/crt0.o)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lm -lgcc -lgloss)

