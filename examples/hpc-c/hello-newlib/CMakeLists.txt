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

project(hello-newlib C ASM)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)
set(NEWLIB /Users/vainogranat/work/newlib-build/build/riscv64-unknown-elf)
#set(LIBGLOSS /Users/vainogranat/work/newlib-build/build/riscv64-unknown-elf)
set(NEWLIB_OBJ ${NEWLIB}/lib/${ARCH}/${ABI})


list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=rv64imac -mabi=lp64 -nostdlib -Fuse-ld=${CMAKE_LINKER} -T${CRT0}/linker.lds --verbose")
#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -march=${ARCH} -mabi=${ABI} -mcmodel=medany -nostdlib -Fuse-ld=${CMAKE_LINKER} -T${CRT0}/linker.lds --verbose")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=${ARCH} -mabi=${ABI} -Fuse-ld=riscv-none-elf-ld -nostdlib -T${CRT0}/linker_sdram.lds -mcmodel=medany ")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add header
#target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${CRT0})

# Add the executable target
#add_executable(${CMAKE_PROJECT_NAME} ${NEWLIB_OBJ}/crt0.o ${SOURCE_FILES} )
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES})

#target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB}/../include)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB_OBJ}/crt0.o)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB_OBJ}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB_OBJ}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${NEWLIB_OBJ}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lgloss -lgcc)
