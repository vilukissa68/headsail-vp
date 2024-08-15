cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(test-dla-driver-ffi C)

# Add C-runtime 0
set(CRT0 "${CMAKE_CURRENT_SOURCE_DIR}/../crt0")


list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED) # CMAKE_C_COMPILER, CMAKE_ASM_COMPILER, CMAKE_LINKER
find_package(DlaDriver REQUIRED) # DLA_INCLUDE_DIR, DLA_LIBRARIES
find_package(Newlib REQUIRED)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Os -march=rv64imac -nostdlib -nostartfiles -mabi=lp64 -fno-builtin -mcmodel=medany -T${CRT0}/linker_sdram.lds")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES} ${LIB_PATH}/crt0.o)

# Invoke DLA to be built
add_dependencies(${CMAKE_PROJECT_NAME} build_dla)

# Add DLA header
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${DLA_INCLUDE_DIR})

# Link bsp static lib
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${DLA_LIBRARIES})

target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lm -lgcc -lgloss)
