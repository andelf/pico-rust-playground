#![no_std]

use core::fmt::Write;

use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use rpi_pico_sdk_sys::*;

use embedded_graphics::fonts::{Font6x12, Font8x16, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egrectangle, egtext, primitive_style, text_style};

use epd::display::DisplayRotation;
use rpi_pico_sdk::gpio::Gpio;
use rpi_pico_sdk::spi::SPI;

use self::bytes::BytesWriter;

pub mod bytes;

/*
GP18 => SCL
GP19 => SDA
GP20 => RES
GP21 => DC
GP22 => CS
GP26 => BUSY
 */

// const PIN_MISO: uint = 16;
const PIN_SCK: uint = 18;
const PIN_MOSI: uint = 19;
const PIN_LED: u32 = 25;
// SPI0_CSn
// const PIN_CS: uint = 17;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    let mut led = Gpio::init(PIN_LED).into_push_pull_output();

    let spi = SPI::spi0(4_000_000);
    // NoMiso
    gpio_set_function(PIN_SCK, GPIO_FUNC_SPI);
    gpio_set_function(PIN_MOSI, GPIO_FUNC_SPI);
    //gpio_set_function(PIN_CS, GPIO_FUNC_SPI);

    let mut rst = Gpio::init(20).into_push_pull_output();
    let dc = Gpio::init(21).into_push_pull_output();
    let cs = Gpio::init(22).into_push_pull_output();
    let busy = Gpio::init(26).into_floating_input();

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

    epd.set_rotation(DisplayRotation::Rotate90);
    //epd.set_mirroring(epd::display::Mirroring::Horizontal);

    epd.init().unwrap();

    epd.clear(BinaryColor::Off).unwrap();
    epd.flush().unwrap();

    sleep_ms(1000);

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
    .draw(&mut epd)
    .unwrap();

    egline!(
        start = (100, 5),
        end = (180, 80),
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1)
    )
    .draw(&mut epd)
    .unwrap();

    Text::new("Hello from Rust!", Point::new(2, 5))
        .into_styled(style)
        .draw(&mut epd)
        .unwrap();

    egrectangle!(
        top_left = Point::new(40, 40),
        bottom_right = Point::new(210, 80),
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1)
    )
    .draw(&mut epd)
    .unwrap();

    egtext!(
        text = "Hello Rust!",
        top_left = (100, 80),
        style = text_style!(
            font = Font8x16,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off
        )
    )
    .draw(&mut epd)
    .unwrap();

    epd.flush().unwrap();

    // epd.sleep().unwrap();

    let mut i = 0;
    let mut buf = [0u8; 1024];
    let mut buf = BytesWriter::new(&mut buf);
    loop {
        // parital refresh
        egrectangle!(
            top_left = (45, 45),
            bottom_right = (200, 70),
            style = primitive_style!(fill_color = BinaryColor::Off)
        )
        .draw(&mut epd)
        .unwrap();

        buf.clear();
        write!(buf, "{}", i).unwrap();

        egtext!(
            text = buf.as_str(),
            top_left = (60, 50),
            style = text_style!(
                font = Font8x16,
                text_color = BinaryColor::On,
                background_color = BinaryColor::Off
            )
        )
        .draw(&mut epd)
        .unwrap();

        epd.flush().unwrap();

        led.toggle().unwrap();
        sleep_ms(1000);
        i += 1;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
