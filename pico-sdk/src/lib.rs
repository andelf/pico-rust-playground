#![no_std]
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code
)]

pub mod ctypes;
use ctypes::*;

mod gen;
pub use gen::*;

// define macros
const sio_hw: *mut sio_hw_t = SIO_BASE as *mut sio_hw_t;

// NOTE: handle static inline functions
#[inline]
pub unsafe fn gpio_pull_up(gpio: uint) {
    gpio_set_pulls(gpio, true, false);
}

/*
    #[inline]
pub fn bool gpio_is_pulled_up(gpio: uint) {
    #[inline]
pub unsafe fn gpio_pull_down(gpio: uint) {
    #[inline]
pub fn bool gpio_is_pulled_down(gpio: uint) {
    #[inline]
pub unsafe fn gpio_disable_pulls(gpio: uint) {
    #[inline]
pub fn bool gpio_get(gpio: uint) {
    #[inline]
pub fn uint32_t gpio_get_all() {
    */
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
    #[inline]
pub unsafe fn gpio_put_all(uint32_t value) {
    */

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
/*   #[inline]
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

/*
    #[inline]
pub fn bool gpio_is_dir_out(gpio: uint) {
    #[inline]
pub fn gpio: uint_get_dir(gpio: uint) {
    */
