add_executable(blink)

add_custom_target(
    rust_blink
    COMMAND cargo build --release -p blink
)
add_dependencies(blink rust_blink)

target_link_libraries(blink
    pico_stdlib
    hardware_pwm
    ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/release/libblink.a
)

# create map/bin/hex file etc.
pico_add_extra_outputs(blink)
