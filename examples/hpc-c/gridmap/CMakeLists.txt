cmake_minimum_required(VERSION 3.10)

set(XLEN 64)
set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")

option(USE_LARGE_DATASET "Use large dataset for gridmap" OFF)
set(RESOLUTION 100 CACHE STRING "Set gridmap resolution")

# Validate that RESOLUTION is an integer
if(NOT RESOLUTION MATCHES "^[0-9]+$")
    message(FATAL_ERROR "RESOLUTION must be an integer")
endif()

if(RESOLUTION EQUAL 100)
    add_definitions(-DRESOLUTION_100CM)
elseif(RESOLUTION EQUAL 50)
    add_definitions(-DRESOLUTION_50CM)
elseif(RESOLUTION EQUAL 25)
    add_definitions(-DRESOLUTION_25CM)
elseif(RESOLUTION EQUAL 12)
    add_definitions(-DRESOLUTION_12CM)
elseif(RESOLUTION EQUAL 5)
    add_definitions(-DRESOLUTION_5CM)
elseif(RESOLUTION EQUAL 1)
    add_definitions(-DRESOLUTION_1CM)
endif()



# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)
set(ARCH rv64imac)
set(ABI lp64)
set(GRIDMAP ${CMAKE_CURRENT_SOURCE_DIR}/tristan-ogm/gridmap)

project(gridmap-headsail C ASM)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)
set(NEWLIB /Users/vainogranat/work/newlib-build/build/riscv64-unknown-elf/lib/${ARCH}/${ABI})
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=${ARCH} -mabi=${ABI} -Fuse-ld=riscv-none-elf-ld -nostdlib -T${CRT0}/linker_sdram.lds -mcmodel=medany")

if(USE_LARGE_DATASET)
  message("Large dataset")
  add_definitions(-DLARGE_DATASET=1)
  add_library(dataset ${GRIDMAP}/src/large_dataset.c)
else()
  message("Small dataset")
  add_library(dataset ${GRIDMAP}/src/dataset.c)
endif()

# Add the executable target
set(GRIDMAP_SOURCES ${GRIDMAP}/src/main.c)
add_executable(${CMAKE_PROJECT_NAME} ${GRIDMAP_SOURCES})

# Link gridmap libraries
add_library(gridmap ${GRIDMAP}/src/gridmap.c)
add_library(datatypes ${GRIDMAP}/src/datatypes.c)
add_library(matrix ${GRIDMAP}/src/matrix.c)
target_link_libraries(gridmap datatypes matrix dataset)

# Newlib
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/crt0.o)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lgloss -lgcc)
