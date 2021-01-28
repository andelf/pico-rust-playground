#![no_std]
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code
)]

pub mod ctypes;
mod gen;
mod gpio;
mod pwm;

use ctypes::*;
pub use gen::*;
pub use gpio::*;
pub use pwm::*;

// hardware/base
#[inline]
unsafe fn hw_set_bits(addr: *mut io_rw_32, mask: uint32_t) {
    let addr = ((addr as u32) | REG_ALIAS_SET_BITS) as *mut uint32_t;
    *addr = mask;
}
#[inline]
unsafe fn hw_clear_bits(addr: *mut io_rw_32, mask: uint32_t) {
    let addr = ((addr as u32) | REG_ALIAS_CLR_BITS) as *mut uint32_t;
    *addr = mask;
}
#[inline]
unsafe fn hw_xor_bits(addr: *mut io_rw_32, mask: uint32_t) {
    let addr = ((addr as u32) | REG_ALIAS_XOR_BITS) as *mut uint32_t;
    *addr = mask;
}
#[inline]
unsafe fn hw_write_masked(addr: *mut io_rw_32, values: uint32_t, write_mask: uint32_t) {
    hw_xor_bits(addr, (*addr ^ values) & write_mask);
}
