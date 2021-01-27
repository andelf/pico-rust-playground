use crate::*;

// C macro definitions
const sio_hw: *mut sio_hw_t = SIO_BASE as *mut sio_hw_t;
// #define padsbank0_hw ((padsbank0_hw_t *)PADS_BANK0_BASE)
const padsbank0_hw: *mut padsbank0_hw_t = PADS_BANK0_BASE as *mut padsbank0_hw_t;

// NOTE: bindgen cannot handle static inline functions
#[inline]
pub unsafe fn gpio_pull_up(gpio: uint) {
    gpio_set_pulls(gpio, true, false);
}

#[inline]
pub unsafe fn gpio_is_pulled_up(gpio: uint) -> bool {
    (*padsbank0_hw).io[gpio as usize] & PADS_BANK0_GPIO0_PUE_BITS != 0
}
#[inline]
pub unsafe fn gpio_pull_down(gpio: uint) {
    gpio_set_pulls(gpio, false, true);
}
#[inline]
pub unsafe fn gpio_is_pulled_down(gpio: uint) -> bool {
    (*padsbank0_hw).io[gpio as usize] & PADS_BANK0_GPIO0_PDE_BITS != 0
}
#[inline]
pub unsafe fn gpio_disable_pulls(gpio: uint) {
    gpio_set_pulls(gpio, false, false);
}
#[inline]
pub unsafe fn gpio_get(gpio: uint) -> bool {
    ((1 << gpio) & (*sio_hw).gpio_in) != 0
}

#[inline]
pub unsafe fn gpio_get_all() -> uint32_t {
    (*sio_hw).gpio_in
}

#[inline]
pub unsafe fn gpio_set_mask(mask: uint32_t) {
    (*sio_hw).gpio_set = mask;
}

#[inline]
pub unsafe fn gpio_clr_mask(mask: uint32_t) {
    (*sio_hw).gpio_clr = mask;
}

#[inline]
pub unsafe fn gpio_xor_mask(mask: uint32_t) {
    (*sio_hw).gpio_togl = mask;
}

/*
#[inline]
pub unsafe fn gpio_put_masked(mask: uint32_t, uint32_t value) {
*/

#[inline]
pub unsafe fn gpio_put_all(value: uint32_t) {
    (*sio_hw).gpio_out = value;
}

#[inline]
pub unsafe fn gpio_put(gpio: uint, value: bool) {
    let mask = 1 << gpio;
    if value {
        gpio_set_mask(mask);
    } else {
        gpio_clr_mask(mask);
    }
}
#[inline]
pub unsafe fn gpio_set_dir_out_masked(mask: uint32_t) {
    (*sio_hw).gpio_oe_set = mask;
}

#[inline]
pub unsafe fn gpio_set_dir_in_masked(mask: uint32_t) {
    (*sio_hw).gpio_oe_clr = mask;
}

#[inline]
pub unsafe fn gpio_set_dir_masked(mask: uint32_t, value: uint32_t) {
    (*sio_hw).gpio_oe_togl = ((*sio_hw).gpio_oe ^ value) & mask;
}

/*
#[inline]
pub unsafe fn gpio_set_dir_all_bits(uint32_t values) {
*/

// NOTE: out changed from bool to u32
#[inline]
pub unsafe fn gpio_set_dir(gpio: uint, out: u32) {
    let mask = 1 << gpio;
    if out != 0 {
        gpio_set_dir_out_masked(mask);
    } else {
        gpio_set_dir_in_masked(mask);
    }
}

#[inline]
pub unsafe fn gpio_is_dir_out(gpio: uint) -> bool {
    (*sio_hw).gpio_oe & (1 << (gpio)) != 0
}

#[inline]
pub unsafe fn gpio_get_dir(gpio: uint) -> uint {
    gpio_is_dir_out(gpio) as uint
}
