#![no_std]

use rpi_pico_sdk_sys::*;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    const PIN_LED: u32 = 25;

    gpio_init(PIN_LED);
    gpio_set_dir(PIN_LED, GPIO_OUT);

    loop {
        gpio_put(PIN_LED, true);
        sleep_ms(500);
        gpio_put(PIN_LED, false);
        sleep_ms(500);
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}