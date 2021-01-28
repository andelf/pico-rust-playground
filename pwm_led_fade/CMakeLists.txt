add_executable(pwm_led_fade)

add_custom_target(
    rust_pwm_led_fade
    COMMAND cargo build --release -p pwm_led_fade
)
add_dependencies(pwm_led_fade rust_pwm_led_fade)

target_link_libraries(pwm_led_fade
    pico_stdlib
    hardware_pwm
    ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/release/libpwm_led_fade.a
)

# create map/bin/hex file etc.
pico_add_extra_outputs(pwm_led_fade)
