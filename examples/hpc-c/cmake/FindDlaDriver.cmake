# Compile & provide library & headers for Headsail DLA Driver
#
# DLA_INCLUDE_DIR := directory with BSP headers
# DLA_LIBRARIES := libraries to be linked

# Add DLA variables
set(DLA_PROJECT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../hpc/dla-driver-ffi)
set(DLA_TARGET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../hpc/dla-driver-ffi/target)
set(DLA_INCLUDE_DIR ${DLA_TARGET_DIR}) # Used by CMake
set(DLA_BUILD_DIR_DEBUG ${DLA_TARGET_DIR}/riscv64imac-unknown-none-elf/debug)
set(DLA_BUILD_DIR_RELEASE ${DLA_TARGET_DIR}/riscv64imac-unknown-none-elf/release)
message(DLA_PROJECT_DIR=${DLA_PROJECT_DIR})

# Build DLA
add_custom_command(
    OUTPUT ${DLA_BUILD_DIR_RELEASE}/libdla_driver_ffi.a
    COMMAND just build-dla
    WORKING_DIRECTORY ${DLA_PROJECT_DIR}
    COMMENT "Building dla driver"
)

add_custom_command(
    OUTPUT ${DLA_BUILD_DIR_DEBUG}/libdla_driver_ffi.a
    COMMAND just build-dla-debug
    WORKING_DIRECTORY ${DLA_PROJECT_DIR}
    COMMENT "Building dla driver"
)



add_custom_target(build_dla DEPENDS ${DLA_BUILD_DIR_RELEASE}/libdla_driver_ffi.a)
add_custom_target(build_dla-debug DEPENDS ${DLA_BUILD_DIR_DEBUG}/libdla_driver_ffi.a)
set(DLA_LIBRARIES ${DLA_BUILD_DIR_RELEASE}/libdla_driver_ffi.a)
