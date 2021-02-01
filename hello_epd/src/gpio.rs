//! Fake embedded-hal gpio via c-sdk.

use core::marker::PhantomData;
use embedded_hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
use rpi_pico_sdk_sys::*;

pub enum GpioFunction {
    XIP = 0,
    SPI = 1,
    UART = 2,
    I2C = 3,
    PWM = 4,
    SIO = 5,
    PIO0 = 6,
    PIO1 = 7,
    GPCK = 8,
    USB = 9,
    NULL = 0xf,
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// On the RP2040, setting both pulls enables a "bus keep" function,
/// i.e. a weak pull to whatever is current high/low state of GPIO.
pub struct BusKeep;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

pub struct Gpio<T> {
    pub pin: uint,
    marker: PhantomData<T>,
}

impl Gpio<Input<Floating>> {
    pub fn init(pin: uint) -> Self {
        // GPIO_FUNC_SIO
        unsafe {
            gpio_init(pin);
        }
        Gpio {
            pin,
            marker: PhantomData,
        }
    }
}

impl<MODE> Gpio<MODE> {
    pub fn into_floating_input(self) -> Gpio<Input<Floating>> {
        let pin = self.pin;
        unsafe {
            gpio_set_dir(pin, GPIO_IN);
            gpio_set_pulls(pin, false, false);
        }
        Gpio {
            pin,
            marker: PhantomData,
        }
    }

    pub fn into_pull_down_input(self) -> Gpio<Input<PullDown>> {
        let pin = self.pin;
        unsafe {
            gpio_set_dir(pin, GPIO_IN);
            gpio_set_pulls(pin, false, true);
        }
        Gpio {
            pin,
            marker: PhantomData,
        }
    }

    pub fn into_pull_up_input(self) -> Gpio<Input<PullUp>> {
        let pin = self.pin;
        unsafe {
            gpio_set_dir(pin, GPIO_IN);
            gpio_set_pulls(pin, true, false);
        }
        Gpio {
            pin,
            marker: PhantomData,
        }
    }

    pub fn into_push_pull_output(self) -> Gpio<Output<PushPull>> {
        let pin = self.pin;
        unsafe {
            gpio_init(pin);
            gpio_set_dir(pin, GPIO_OUT);
            // gpio_set_pulls(pin, false, false);
        }
        Gpio {
            pin,
            marker: PhantomData,
        }
    }
}

impl<MODE> OutputPin for Gpio<Output<MODE>> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            gpio_put(self.pin, true);
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            gpio_put(self.pin, false);
        }
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for Gpio<Output<MODE>> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) })
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) == false })
    }
}

impl<MODE> toggleable::Default for Gpio<Output<MODE>> {}

impl<MODE> InputPin for Gpio<Output<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) })
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) == false })
    }
}

impl<MODE> InputPin for Gpio<Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) })
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { gpio_get(self.pin) == false })
    }
}
