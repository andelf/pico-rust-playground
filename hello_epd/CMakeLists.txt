add_executable(hello_epd)

add_custom_target(
    rust_hello_epd
    COMMAND cargo build -p hello_epd
)
add_dependencies(hello_epd rust_hello_epd)

target_link_libraries(hello_epd
    pico_stdlib
    #hardware_pwm
    hardware_spi
    ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/debug/libhello_epd.a
)

# create map/bin/hex file etc.
pico_add_extra_outputs(hello_epd)
