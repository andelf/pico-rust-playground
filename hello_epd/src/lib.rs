#![no_std]

use embedded_hal::digital::v2::ToggleableOutputPin;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rpi_pico_sdk_sys::*;

use embedded_graphics::fonts::{Font6x12, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, Rectangle};
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::{egcircle, egline, egrectangle, egtext, primitive_style, text_style};

pub mod gpio;
pub mod spi;

use gpio::Gpio;
use spi::SPI;

/*
GP18 => SCL
GP19 => SDA
GP20 => RES
GP21 => DC
GP22 => CS
GP26 => BUSY
 */

const PIN_MISO: uint = 16;
const PIN_SCK: uint = 18;
const PIN_MOSI: uint = 19;
const PIN_LED: u32 = 25;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    let mut led = Gpio::init(PIN_LED).into_push_pull_output();

    let mut spi = SPI::spi0(1000 * 1000);
    gpio_set_function(PIN_MISO, GPIO_FUNC_SPI);
    gpio_set_function(PIN_SCK, GPIO_FUNC_SPI);
    gpio_set_function(PIN_MOSI, GPIO_FUNC_SPI);

    let mut rst = Gpio::init(20).into_push_pull_output();
    let mut dc = Gpio::init(21).into_push_pull_output();
    let mut cs = Gpio::init(22).into_push_pull_output();
    let mut busy = Gpio::init(26).into_floating_input();

    cs.set_high().unwrap();

    let di = epd::Interface::new(spi, dc, cs, busy);
    let mut epd = epd::display::EPaperDisplay::<_, epd::display::DisplaySize250x122>::new(di);

    // epd
    rst.set_high().unwrap();
    sleep_ms(200);
    rst.set_low().unwrap();
    sleep_ms(10);
    rst.set_high().unwrap();
    sleep_ms(200);
    rst.set_high().unwrap();

    epd.set_rotation(epd::display::DisplayRotation::Rotate270);
    //epd.set_mirroring(epd::display::Mirroring::Horizontal);

    sleep_ms(2000);
    epd.init();

    epd.clear(BinaryColor::Off);
    epd.flush().unwrap();

    let style = text_style!(
        font = Font6x12,
        text_color = BinaryColor::On,
        background_color = BinaryColor::Off
    );

    egcircle!(
        center = (40, 40),
        radius = 35,
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 2)
    )
    .draw(&mut epd);

    egline!(
        start = (100, 5),
        end = (180, 80),
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 4)
    )
    .draw(&mut epd);

    Text::new("Hello from Rust!", Point::new(0, 5))
        .into_styled(style)
        .draw(&mut epd)
        .unwrap();
    epd.flush().unwrap();

    loop {
        led.toggle().unwrap();
        sleep_ms(1000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
