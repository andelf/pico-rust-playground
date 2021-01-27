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

use ctypes::*;
pub use gen::*;
pub use gpio::*;
