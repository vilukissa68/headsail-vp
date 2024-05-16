# Compile & provide library & headers for Headsail BSP
#
# BSP_INCLUDE_DIR := directory with BSP headers
# BSP_LIBRARIES := libraries to be linked

# Add BSP variables
set(BSP_PROJECT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../headsail-bsp-ffi)
set(BSP_TARGET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../headsail-bsp-ffi/target)
set(BSP_INCLUDE_DIR ${BSP_TARGET_DIR}) # Used by CMake
set(BSP_BUILD_DIR_DEBUG ${BSP_TARGET_DIR}/riscv64imac-unknown-none-elf/debug)
set(BSP_BUILD_DIR_RELEASE ${BSP_TARGET_DIR}/riscv64imac-unknown-none-elf/release)
message(BSP_PROJECT_DIR=${BSP_PROJECT_DIR})

# Build BSP
add_custom_command(
    OUTPUT ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a
    COMMAND cd ${BSP_PROJECT_DIR} && just build-hpc
    WORKING_DIRECTORY ${BSP_PROJECT_DIR}
    COMMENT "Building bsp"
)

add_custom_target(build_hpc DEPENDS ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a)
set(BSP_LIBRARIES ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a)
