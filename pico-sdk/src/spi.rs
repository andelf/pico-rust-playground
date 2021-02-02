//! Fake embedded-hal SPI via c-sdk.

use rpi_pico_sdk_sys::*;
use core::convert::Infallible;
use embedded_hal::blocking::spi;

pub struct SPI {
    hw: *mut spi_inst_t,
}

impl Drop for SPI {
    fn drop(&mut self) {
        unsafe {
            spi_deinit(self.hw);
        }
    }
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

    pub fn set_baudrate(&mut self, baudrate: uint) {
        unsafe {
            spi_set_baudrate(self.hw, baudrate);
        }
    }
}

impl spi::Write<u8> for SPI {
    type Error = Infallible;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        unsafe {
            spi_write_blocking(self.hw, words.as_ptr(), words.len() as _);
        }
        Ok(())
    }
}
