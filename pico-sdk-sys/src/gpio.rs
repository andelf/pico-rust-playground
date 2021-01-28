use crate::*;

// C macro definitions
const sio_hw: *mut sio_hw_t = SIO_BASE as *mut sio_hw_t;
// #define padsbank0_hw ((padsbank0_hw_t *)PADS_BANK0_BASE)
const padsbank0_hw: *mut padsbank0_hw_t = PADS_BANK0_BASE as *mut padsbank0_hw_t;

/// \\brief Get state of a single specified GPIO
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
/// \\return Current state of the GPIO. 0 for low, non-zero for high
#[inline]
pub unsafe fn gpio_get(gpio: uint) -> bool {
    ((1 << gpio) & (*sio_hw).gpio_in) != 0
}

/// \brief Get raw value of all GPIOs
///  \ingroup hardware_gpio
///
/// \return Bitmask of raw GPIO values, as bits 0-29
#[inline]
pub unsafe fn gpio_get_all() -> u32 {
    (*sio_hw).gpio_in
}

/// \brief Drive high every GPIO appearing in mask
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO values to set, as bits 0-29
#[inline]
pub unsafe fn gpio_set_mask(mask: u32) {
    (*sio_hw).gpio_set = mask;
}

/// \brief Drive low every GPIO appearing in mask
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO values to clear, as bits 0-29
#[inline]
pub unsafe fn gpio_clr_mask(mask: u32) {
    (*sio_hw).gpio_clr = mask;
}

/// \brief Toggle every GPIO appearing in mask
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO values to toggle, as bits 0-29
#[inline]
pub unsafe fn gpio_xor_mask(mask: u32) {
    (*sio_hw).gpio_togl = mask;
}

/// \brief Drive GPIO high/low depending on parameters
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO values to change, as bits 0-29
/// \param value Value to set
///
/// For each 1 bit in \p mask, drive that pin to the value given by
/// corresponding bit in \p value, leaving other pins unchanged.
/// Since this uses the TOGL alias, it is concurrency-safe with e.g. an IRQ
/// bashing different pins from the same core.
#[inline]
pub unsafe fn gpio_put_masked(mask: u32, value: u32) {
    (*sio_hw).gpio_togl = ((*sio_hw).gpio_out ^ value) & mask;
}

/// \brief Drive all pins simultaneously
///  \ingroup hardware_gpio
///
/// \param value Bitmask of GPIO values to change, as bits 0-29
#[inline]
pub unsafe fn gpio_put_all(value: u32) {
    (*sio_hw).gpio_out = value;
}

/// \brief Drive a single GPIO high/low
///  \ingroup hardware_gpio
///
/// \param gpio GPIO number
/// \param value If false clear the GPIO, otherwise set it.
#[inline]
pub unsafe fn gpio_put(gpio: uint, value: bool) {
    let mask = 1 << gpio;
    if value {
        gpio_set_mask(mask);
    } else {
        gpio_clr_mask(mask);
    }
}

/// \brief Set a number of GPIOs to output
///  \ingroup hardware_gpio
///
/// Switch all GPIOs in \"mask\" to output
///
/// \param mask Bitmask of GPIO to set to output, as bits 0-29
#[inline]
pub unsafe fn gpio_set_dir_out_masked(mask: u32) {
    (*sio_hw).gpio_oe_set = mask;
}

/// \brief Set a number of GPIOs to input
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO to set to input, as bits 0-29
#[inline]
pub unsafe fn gpio_set_dir_in_masked(mask: u32) {
    (*sio_hw).gpio_oe_clr = mask;
}

/// \brief Set multiple GPIO directions
///  \ingroup hardware_gpio
///
/// \param mask Bitmask of GPIO to set to input, as bits 0-29
/// \param value Values to set
///
/// For each 1 bit in \"mask\", switch that pin to the direction given by
/// corresponding bit in \"value\", leaving other pins unchanged.
/// E.g. gpio_set_dir_masked(0x3, 0x2); -> set pin 0 to input, pin 1 to output,
/// simultaneously.
#[inline]
pub unsafe fn gpio_set_dir_masked(mask: u32, value: u32) {
    (*sio_hw).gpio_oe_togl = ((*sio_hw).gpio_oe ^ value) & mask;
}

/// \brief Set direction of all pins simultaneously.
///  \ingroup hardware_gpio
///
/// \param values individual settings for each gpio; for GPIO N, bit N is 1 for out, 0 for in
#[inline]
pub unsafe fn gpio_set_dir_all_bits(values: u32) {
    (*sio_hw).gpio_oe = values;
}

/// \brief Set a single GPIO direction
///  \ingroup hardware_gpio
///
/// \param gpio GPIO number
/// \param out true for out, false for in
// NOTE: out is a bool
#[inline]
pub unsafe fn gpio_set_dir(gpio: uint, out: u32) {
    let mask = 1 << gpio;
    if out != 0 {
        gpio_set_dir_out_masked(mask);
    } else {
        gpio_set_dir_in_masked(mask);
    }
}

/// \brief Check if a specific GPIO direction is OUT
///  \ingroup hardware_gpio
///
/// \param gpio GPIO number
/// \return true if the direction for the pin is OUT
#[inline]
pub unsafe fn gpio_is_dir_out(gpio: uint) -> bool {
    (*sio_hw).gpio_oe & (1 << (gpio)) != 0
}

/// \brief Get a specific GPIO direction
///  \ingroup hardware_gpio
///
/// \param gpio GPIO number
/// \return 1 for out, 0 for in
#[inline]
pub unsafe fn gpio_get_dir(gpio: uint) -> uint {
    gpio_is_dir_out(gpio) as uint
}

/// \\brief Set specified GPIO to be pulled up.
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
pub unsafe fn gpio_pull_up(gpio: uint) {
    gpio_set_pulls(gpio, true, false);
}
/// \\brief Determine if the specified GPIO is pulled up.
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
/// \\return true if the GPIO is pulled up
pub unsafe fn gpio_is_pulled_up(gpio: uint) -> bool {
    (*padsbank0_hw).io[gpio as usize] & PADS_BANK0_GPIO0_PUE_BITS != 0
}
/// \\brief Set specified GPIO to be pulled down.
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
pub unsafe fn gpio_pull_down(gpio: uint) {
    gpio_set_pulls(gpio, false, true);
}
/// \\brief Determine if the specified GPIO is pulled down.
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
/// \\return true if the GPIO is pulled down
pub unsafe fn gpio_is_pulled_down(gpio: uint) -> bool {
    (*padsbank0_hw).io[gpio as usize] & PADS_BANK0_GPIO0_PDE_BITS != 0
}
/// \\brief Disable pulls on specified GPIO
///  \\ingroup hardware_gpio
///
/// \\param gpio GPIO number
pub unsafe fn gpio_disable_pulls(gpio: uint) {
    gpio_set_pulls(gpio, false, false);
}
