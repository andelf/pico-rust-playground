#![no_std]

use embedded_hal::digital::v2::ToggleableOutputPin;
use rpi_pico_sdk_sys::*;

mod spi;
pub mod gpio;

use gpio::Gpio;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    const PIN_LED: u32 = 25;

    let mut led = Gpio::init(PIN_LED).into_push_pull_output();
    loop {
        led.toggle().unwrap();
        sleep_ms(100);
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
