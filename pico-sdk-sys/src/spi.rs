use crate::*;

// #define spi0_hw ((spi_hw_t *const)SPI0_BASE)
// #define spi1_hw ((spi_hw_t *const)SPI1_BASE)
const spi0_hw: *mut spi_hw_t = SPI0_BASE as _;
const spi1_hw: *mut spi_hw_t = SPI1_BASE as _;

pub const spi0: *mut spi_inst_t = spi0_hw as *mut spi_inst_t;
pub const spi1: *mut spi_inst_t = spi1_hw as *mut spi_inst_t;

/// \\brief Convert I2c instance to hardware instance number
///  \\ingroup hardware_spi
///
/// \\param spi SPI instance
/// \\return Number of SPI, 0 or 1.
#[inline]
pub unsafe fn spi_get_index(spi: *mut spi_inst_t) -> uint {
    (spi == spi1_hw as *mut spi_inst_t) as uint
}
#[inline]
pub unsafe fn spi_get_hw(spi: *mut spi_inst_t) -> *mut spi_hw_t {
    // NOTE: skip check
    spi as *mut spi_hw_t
}
/// \\brief Configure SPI
///  \\ingroup hardware_spi
///
/// Configure how the SPI serialises and deserialises data on the wire
///
/// \\param spi SPI instance specifier, either \\ref spi0 or \\ref spi1
/// \\param data_bits Number of data bits per transfer. Valid values 4..16.
/// \\param cpol SSPCLKOUT polarity, applicable to Motorola SPI frame format only.
/// \\param cpha SSPCLKOUT phase, applicable to Motorola SPI frame format only
/// \\param order Must be SPI_MSB_FIRST, no other values supported on the PL022
#[inline]
pub unsafe fn spi_set_format(
    spi: *mut spi_inst_t,
    data_bits: uint,
    cpol: spi_cpol_t,
    cpha: spi_cpha_t,
    _order: spi_order_t,
) {
    // LSB-first not supported on PL022
    hw_write_masked(
        &mut (*spi_get_hw(spi)).cr0,
        (data_bits - 1) << SPI_SSPCR0_DSS_LSB
            | cpol << SPI_SSPCR0_SPO_LSB
            | cpha << SPI_SSPCR0_SPH_LSB,
        SPI_SSPCR0_DSS_BITS | SPI_SSPCR0_SPO_BITS | SPI_SSPCR0_SPH_BITS,
    );
}
/// \\brief Set SPI master/slave
///  \\ingroup hardware_spi
///
/// Configure the SPI for master- or slave-mode operation. By default,
/// spi_init() sets master-mode.
///
/// \\param spi SPI instance specifier, either \\ref spi0 or \\ref spi1
/// \\param slave true to set SPI device as a slave device, false for master.
pub unsafe fn spi_set_slave(spi: *mut spi_inst_t, slave: bool) {
    if slave {
        hw_set_bits(&mut (*spi_get_hw(spi)).cr1, SPI_SSPCR1_MS_BITS);
    } else {
        hw_clear_bits(&mut (*spi_get_hw(spi)).cr1, SPI_SSPCR1_MS_BITS);
    }
}
/// \\brief Check whether a write can be done on SPI device
///  \\ingroup hardware_spi
///
/// \\param spi SPI instance specifier, either \\ref spi0 or \\ref spi1
/// \\return 0 if no space is available to write. Non-zero if a write is possible
///
/// \\note Although the controllers each have a 8 deep TX FIFO, the current HW implementation can only return 0 or 1
/// rather than the space available.
pub unsafe fn spi_is_writable(spi: *mut spi_inst_t) -> bool {
    (((*spi_get_hw(spi)).sr & SPI_SSPSR_TNF_BITS) >> SPI_SSPSR_TNF_LSB) != 0
}
/// \\brief Check whether a read can be done on SPI device
///  \\ingroup hardware_spi
///
/// \\param spi SPI instance specifier, either \\ref spi0 or \\ref spi1
/// \\return Non-zero if a read is possible i.e. data is present
///
/// \\note Although the controllers each have a 8 deep RX FIFO, the current HW implementation can only return 0 or 1
/// rather than the data available.
pub unsafe fn spi_is_readable(spi: *mut spi_inst_t) -> bool {
    (((*spi_get_hw(spi)).sr & SPI_SSPSR_RNE_BITS) >> SPI_SSPSR_RNE_LSB) != 0
}
