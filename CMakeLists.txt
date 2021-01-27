cmake_minimum_required(VERSION 3.12)

# Pull in PICO SDK (must be before project)
include($ENV{PICO_SDK_PATH}/external/pico_sdk_import.cmake)

project(playground C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

pico_sdk_init()

# The executable
add_executable(main
  main.c
)

add_custom_target(
    lib_rust_wrapper
    COMMAND cargo build --release
)
add_dependencies(main lib_rust_wrapper)

# Pull in our pico_stdlib which pulls in commonly used features
target_link_libraries(main pico_stdlib
    ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/release/librust_wrapper.a)

# create map/bin/hex file etc.
pico_add_extra_outputs(main)
