use rpi_pico_sdk_sys::*;

const spi0: *mut spi_inst_t = 0;
const spi1: *mut spi_inst_t = 0;
pub struct SPI {
    hw: *mut spi_inst_t,
}

impl SPI {
    pub fn spi0(baudrate: uint) -> Self {
        let mut s = SPI { hw: spi0 };
        s.init(baudrate);
        s
    }

    pub fn spi1(baudrate: uint) -> Self {
        let mut s = SPI { hw: spi1 };
        s.init(baudrate);
        s
    }

    fn init(&mut self, baudrate: uint) {
        unsafe {
            spi_init(self.hw, baudrate);
        }
    }
}
