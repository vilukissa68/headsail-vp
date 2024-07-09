cmake_minimum_required(VERSION 3.10)

set(XLEN 64)
set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
# set(CMAKE_ASM_COMPILER $ENV{CC}/../riscv${XLEN}-unknown-elf-as)
# set(CMAKE_LINKER $ENV{CC}/../riscv${XLEN}-unknown-elf-ld)

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

# Add the executable target
set(GRIDMAP_SOURCES ${GRIDMAP}/src/main.c)
add_executable(${CMAKE_PROJECT_NAME} ${GRIDMAP_SOURCES})

# Link gridmap libraries
add_library(gridmap ${GRIDMAP}/src/gridmap.c)
add_library(datatypes ${GRIDMAP}/src/datatypes.c)
add_library(matrix ${GRIDMAP}/src/matrix.c)
add_library(dataset ${GRIDMAP}/src/dataset.c)
target_link_libraries(gridmap datatypes matrix dataset)

# Newlib
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/crt0.o)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lgloss -lgcc)
