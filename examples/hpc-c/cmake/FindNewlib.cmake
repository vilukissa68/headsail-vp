list(APPEND CMAKE_PREFIX_PATH "/opt/headsail-newlib" "~/work/newlib-build/build")

if(NOT NEWLIB_PATH)
  find_path(
    NEWLIB_PATH
    NAMES "include/" "lib/"
    PATH_SUFFIXES "build/riscv64-unknown-elf" "riscv64-unknown-elf" "cygwin-newlib/riscv64-unknown-elf"
  )  
  if(NOT NEWLIB_PATH)
    message(FATAL_ERROR "Newlib not found")
  endif()
endif()

set(INC_PATH "${NEWLIB_PATH}/include")
set(LIB_PATH "${NEWLIB_PATH}/lib")