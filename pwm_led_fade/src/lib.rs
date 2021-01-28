#![no_std]

use rpi_pico_sdk_sys::*;
use rpi_pico_sdk_sys::ctypes::c_int;


unsafe extern "C" fn on_pwm_wrap() {
    static mut FADE: i32 = 0;
    static mut GOING_UP: bool  = true;
    // Clear the interrupt flag that brought us here
    pwm_clear_irq(pwm_gpio_to_slice_num(PICO_DEFAULT_LED_PIN));

    if GOING_UP {
        FADE += 1;
        if FADE > 255 {
            FADE = 255;
            GOING_UP = false;
        }
    } else {
        FADE -= 1;
        if FADE < 0 {
            FADE = 0;
            GOING_UP = true;
        }
    }
    // Square the FADE value to make the LED's brightness appear more linear
    // Note this range matches with the wrap value
    pwm_set_gpio_level(PICO_DEFAULT_LED_PIN, (FADE * FADE) as u16);
}


#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    // Tell the LED pin that the PWM is in charge of its value.
    gpio_set_function(PICO_DEFAULT_LED_PIN, GPIO_FUNC_PWM);
    // Figure out which slice we just connected to the LED pin
    let slice_num = pwm_gpio_to_slice_num(PICO_DEFAULT_LED_PIN);

    // Mask our slice's IRQ output into the PWM block's single interrupt line,
    // and register our interrupt handler
    pwm_clear_irq(slice_num);
    pwm_set_irq_enabled(slice_num, true);
    irq_set_exclusive_handler(PWM_IRQ_WRAP, Some(on_pwm_wrap));
    irq_set_enabled(PWM_IRQ_WRAP, true);


    // Get some sensible defaults for the slice configuration. By default, the
    // counter is allowed to wrap over its maximum range (0 to 2**16-1)
    let mut config = pwm_get_default_config();
    // Set divider, reduces counter clock to sysclock/this value
    pwm_config_set_clkdiv_int(&mut config, 4);

    // Load the configuration into our PWM slice, and set it running.
    pwm_init(slice_num, &config, true);


    loop {
        sleep_ms(500);
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}