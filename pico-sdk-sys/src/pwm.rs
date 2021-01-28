use core::mem;

use crate::*;

// #define pwm_hw ((pwm_hw_t *const)PWM_BASE)
const pwm_hw: *mut pwm_hw_t = PWM_BASE as _;

/// \\brief Determine the PWM slice that is attached to the specified GPIO
///  \\ingroup hardware_pwm
///
/// \\return The PWM slice number that controls the specified GPIO.
#[inline]
pub unsafe fn pwm_gpio_to_slice_num(gpio: uint) -> uint {
    (gpio >> 1) & 7
}
/// \\brief Determine the PWM channel that is attached to the specified GPIO.
///  \\ingroup hardware_pwm
///
/// Each slice 0 to 7 has two channels, A and B.
///
/// \\return The PWM channel that controls the specified GPIO.
#[inline]
pub unsafe fn pwm_gpio_to_channel(gpio: uint) -> uint {
    gpio & 1
}
/// \\brief Set phase correction in a PWM configuration
///  \\ingroup hardware_pwm
///
/// \\param c PWM configuration struct to modify
/// \\param phase_correct true to set phase correct modulation, false to set trailing edge
///
/// Setting phase control to true means that instead of wrapping back to zero when the wrap point is reached,
/// the PWM starts counting back down. The output frequency is halved when phase-correct mode is enabled.
#[inline]
pub unsafe fn pwm_config_set_phase_correct(c: &mut pwm_config, phase_correct: bool) {
    c.csr = (c.csr & !PWM_CH0_CSR_PH_CORRECT_BITS)
        | ((phase_correct as u32) << PWM_CH0_CSR_PH_CORRECT_LSB);
}
/// \\brief Set clock divider in a PWM configuration
///  \\ingroup hardware_pwm
///
/// \\param c PWM configuration struct to modify
/// \\param div Value to divide counting rate by. Must be greater than or equal to 1.
///
/// If the divide mode is free-running, the PWM counter runs at clk_sys / div.
/// Otherwise, the divider reduces the rate of events seen on the B pin input (level or edge)
/// before passing them on to the PWM counter.
#[inline]
pub unsafe fn pwm_config_set_clkdiv(c: &mut pwm_config, div: f32) {
    c.div = (div * (1 << PWM_CH1_DIV_INT_LSB) as f32) as u32;
}
/// \\brief Set PWM clock divider in a PWM configuration
///  \\ingroup hardware_pwm
///
/// \\param c PWM configuration struct to modify
/// \\param div integer value to reduce counting rate by. Must be greater than or equal to 1.
///
/// If the divide mode is free-running, the PWM counter runs at clk_sys / div.
/// Otherwise, the divider reduces the rate of events seen on the B pin input (level or edge)
/// before passing them on to the PWM counter.
#[inline]
pub unsafe fn pwm_config_set_clkdiv_int(c: &mut pwm_config, div: uint) {
    c.div = div << PWM_CH1_DIV_INT_LSB;
}
/// \\brief Set PWM counting mode in a PWM configuration
///  \\ingroup hardware_pwm
///
/// \\param c PWM configuration struct to modify
/// \\param mode PWM divide/count mode
///
/// Configure which event gates the operation of the fractional divider.
/// The default is always-on (free-running PWM). Can also be configured to count on
/// high level, rising edge or falling edge of the B pin input.
#[inline]
pub unsafe fn pwm_config_set_clkdiv_mode(c: &mut pwm_config, mode: pwm_clkdiv_mode) {
    c.csr = (c.csr & !PWM_CH0_CSR_DIVMODE_BITS) | (mode << PWM_CH0_CSR_DIVMODE_LSB);
}
/// \\brief Set output polarity in a PWM configuration
///  \\ingroup hardware_pwm
///
/// \\param c PWM configuration struct to modify
/// \\param a true to invert output A
/// \\param b true to invert output B
#[inline]
pub unsafe fn pwm_config_set_output_polarity(c: &mut pwm_config, a: bool, b: bool) {
    c.csr = (c.csr & !(PWM_CH0_CSR_A_INV_BITS | PWM_CH0_CSR_B_INV_BITS))
        | (((a as u32) << PWM_CH0_CSR_A_INV_LSB) | ((b as u32) << PWM_CH0_CSR_B_INV_LSB));
}
/// \\brief Set PWM counter wrap value in a PWM configuration
///  \\ingroup hardware_pwm
///
/// Set the highest value the counter will reach before returning to 0. Also known as TOP.
///
/// \\param c PWM configuration struct to modify
/// \\param wrap Value to set wrap to
#[inline]
pub unsafe fn pwm_config_set_wrap(c: &mut pwm_config, wrap: u16) {
    c.top = wrap as u32;
}
/// \\brief Initialise a PWM with settings from a configuration object
///  \\ingroup hardware_pwm
///
/// Use the \\ref pwm_get_default_config() function to initialise a config structure, make changes as
/// needed using the pwm_config_* functions, then call this function to set up the PWM.
///
/// \\param slice_num PWM slice number
/// \\param c The configuration to use
/// \\param start If true the PWM will be started running once configured. If false you will need to start
///  manually using \\ref pwm_set_enabled() or \\ref pwm_set_mask_enabled()
#[inline]
pub unsafe fn pwm_init(slice_num: uint, c: &pwm_config, start: bool) {
    let slice_num = slice_num as usize;
    (*pwm_hw).slice[slice_num].csr = 0;

    (*pwm_hw).slice[slice_num].ctr = PWM_CH0_CTR_RESET;
    (*pwm_hw).slice[slice_num].cc = PWM_CH0_CC_RESET;
    (*pwm_hw).slice[slice_num].top = c.top;
    (*pwm_hw).slice[slice_num].div = c.div;
    (*pwm_hw).slice[slice_num].csr = c.csr | ((start as u32) << PWM_CH0_CSR_EN_LSB);
}
/// \\brief Get a set of default values for PWM configuration
///  \\ingroup hardware_pwm
///
/// PWM config is free running at system clock speed, no phase correction, wrapping at 0xffff,
/// with standard polarities for channels A and B.
///
/// \\return Set of default values.
#[inline]
pub unsafe fn pwm_get_default_config() -> pwm_config {
    let mut c: pwm_config = mem::zeroed();
    pwm_config_set_phase_correct(&mut c, false);
    pwm_config_set_clkdiv_int(&mut c, 1);
    pwm_config_set_clkdiv_mode(&mut c, PWM_DIV_FREE_RUNNING);
    pwm_config_set_output_polarity(&mut c, false, false);
    pwm_config_set_wrap(&mut c, 0xffff_u16);
    c
}
/// \\brief Set the current PWM counter wrap value
///  \\ingroup hardware_pwm
///
/// Set the highest value the counter will reach before returning to 0. Also known as TOP.
///
/// \\param slice_num PWM slice number
/// \\param wrap Value to set wrap to
#[inline]
pub unsafe fn pwm_set_wrap(slice_num: uint, wrap: u16) {
    (*pwm_hw).slice[slice_num as usize].top = wrap as u32;
}
/// \\brief Set the current PWM counter compare value for one channel
///  \\ingroup hardware_pwm
///
/// Set the value of the PWM counter compare value, for either channel A or channel B
///
/// \\param slice_num PWM slice number
/// \\param chan Which channel to update. 0 for A, 1 for B.
/// \\param level new level for the selected output
#[inline]
pub unsafe fn pwm_set_chan_level(slice_num: uint, chan: uint, level: u16) {
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num as usize].cc,
        (level as u32)
            << (if chan != 0 {
                PWM_CH0_CC_B_LSB
            } else {
                PWM_CH0_CC_A_LSB
            }),
        if chan != 0 {
            PWM_CH0_CC_B_BITS
        } else {
            PWM_CH0_CC_A_BITS
        },
    );
}
/// \\brief Set PWM counter compare values
///  \\ingroup hardware_pwm
///
/// Set the value of the PWM counter compare values, A and B
///
/// \\param slice_num PWM slice number
/// \\param level_a Value to set compare A to. When the counter reaches this value the A output is deasserted
/// \\param level_b Value to set compare B to. When the counter reaches this value the B output is deasserted
#[inline]
pub unsafe fn pwm_set_both_levels(slice_num: uint, level_a: u16, level_b: u16) {
    (*pwm_hw).slice[slice_num as usize].cc =
        ((level_b as u32) << PWM_CH0_CC_B_LSB) | ((level_a as u32) << PWM_CH0_CC_A_LSB);
}
/// \\brief Helper function to set the PWM level for the slice and channel associated with a GPIO.
///  \\ingroup hardware_pwm
///
/// Look up the correct slice (0 to 7) and channel (A or B) for a given GPIO, and update the corresponding
/// counter-compare field.
///
/// This PWM slice should already have been configured and set running. Also be careful of multiple GPIOs
/// mapping to the same slice and channel (if GPIOs have a difference of 16).
///
/// \\param gpio GPIO to set level of
/// \\param level PWM level for this GPIO
#[inline]
pub unsafe fn pwm_set_gpio_level(gpio: uint, level: u16) {
    pwm_set_chan_level(
        pwm_gpio_to_slice_num(gpio),
        pwm_gpio_to_channel(gpio),
        level,
    );
}
/// \\brief Get PWM counter
///  \\ingroup hardware_pwm
///
/// Get current value of PWM counter
///
/// \\param slice_num PWM slice number
/// \\return Current value of PWM counter
#[inline]
pub unsafe fn pwm_get_counter(slice_num: uint) -> i16 {
    (*pwm_hw).slice[slice_num as usize].ctr as i16
}
/// \\brief Set PWM counter
///  \\ingroup hardware_pwm
///
/// Set the value of the PWM counter
///
/// \\param slice_num PWM slice number
/// \\param c Value to set the PWM counter to
///
#[inline]
pub unsafe fn pwm_set_counter(slice_num: uint, c: u16) {
    (*pwm_hw).slice[slice_num as usize].ctr = c as u32;
}
/// \\brief Advance PWM count
///  \\ingroup hardware_pwm
///
/// Advance the phase of a running the counter by 1 count.
///
/// This function will return once the increment is complete.
///
/// \\param slice_num PWM slice number
#[inline]
pub unsafe fn pwm_advance_count(slice_num: uint) {
    hw_set_bits(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        PWM_CH0_CSR_PH_ADV_BITS,
    );
    while ((*pwm_hw).slice[slice_num as usize].csr & PWM_CH0_CSR_PH_ADV_BITS) != 0 {}
}
/// \\brief Retard PWM count
///  \\ingroup hardware_pwm
///
/// Retard the phase of a running counter by 1 count
///
/// This function will return once the retardation is complete.
///
/// \\param slice_num PWM slice number
#[inline]
pub unsafe fn pwm_retard_count(slice_num: uint) {
    hw_set_bits(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        PWM_CH0_CSR_PH_RET_BITS,
    );
    while ((*pwm_hw).slice[slice_num as usize].csr & PWM_CH0_CSR_PH_RET_BITS) != 0 {}
}
/// \\brief Set PWM clock divider using an 8:4 fractional value
///  \\ingroup hardware_pwm
///
/// Set the clock divider. Counter increment will be on sysclock divided by this value, taking in to account the gating.
///
/// \\param slice_num PWM slice number
/// \\param integer  8 bit integer part of the clock divider
/// \\param fract 4 bit fractional part of the clock divider
#[inline]
pub unsafe fn pwm_set_clkdiv_int_frac(slice_num: uint, integer: u8, fract: u8) {
    (*pwm_hw).slice[slice_num as usize].div =
        ((integer as u32) << PWM_CH0_DIV_INT_LSB) | ((fract as u32) << PWM_CH0_DIV_FRAC_LSB);
}
/// \\brief Set PWM clock divider
///  \\ingroup hardware_pwm
///
/// Set the clock divider. Counter increment will be on sysclock divided by this value, taking in to account the gating.
///
/// \\param slice_num PWM slice number
/// \\param divider Floating point clock divider,  1.f <= value < 256.f
#[inline]
pub unsafe fn pwm_set_clkdiv(slice_num: uint, divider: f32) {
    let i = divider as u8;
    let f = (divider - i as f32) * ((0x01 << 4) as f32);
    pwm_set_clkdiv_int_frac(slice_num, i, f as u8);
}
/// \\brief Set PWM output polarity
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
/// \\param a true to invert output A
/// \\param b true to invert output B
#[inline]
pub unsafe fn pwm_set_output_polarity(slice_num: uint, a: bool, b: bool) {
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        (a as u32) << PWM_CH0_CSR_A_INV_LSB | (b as u32) << PWM_CH0_CSR_B_INV_LSB,
        PWM_CH0_CSR_A_INV_BITS | PWM_CH0_CSR_B_INV_BITS,
    );
}
/// \\brief Set PWM divider mode
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
/// \\param mode Required divider mode
#[inline]
pub unsafe fn pwm_set_clkdiv_mode(slice_num: uint, mode: pwm_clkdiv_mode) {
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        mode << PWM_CH0_CSR_DIVMODE_LSB,
        PWM_CH0_CSR_DIVMODE_BITS,
    );
}
/// \\brief Set PWM phase correct on/off
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
/// \\param phase_correct true to set phase correct modulation, false to set trailing edge
///
/// Setting phase control to true means that instead of wrapping back to zero when the wrap point is reached,
/// the PWM starts counting back down. The output frequency is halved when phase-correct mode is enabled.
#[inline]
pub unsafe fn pwm_set_phase_correct(slice_num: uint, phase_correct: bool) {
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        (phase_correct as u32) << PWM_CH0_CSR_PH_CORRECT_LSB,
        PWM_CH0_CSR_PH_CORRECT_BITS,
    );
}
/// \\brief Enable/Disable PWM
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
/// \\param enabled true to enable the specified PWM, false to disable
#[inline]
pub unsafe fn pwm_set_enabled(slice_num: uint, enabled: bool) {
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num as usize].csr,
        (enabled as u32) << PWM_CH0_CSR_EN_LSB,
        PWM_CH0_CSR_EN_BITS,
    );
}
/// \\brief Enable/Disable multiple PWM slices simultaneously
///  \\ingroup hardware_pwm
///
/// \\param mask Bitmap of PWMs to enable/disable. Bits 0 to 7 enable slices 0-7 respectively
#[inline]
pub unsafe fn pwm_set_mask_enabled(mask: u32) {
    (*pwm_hw).en = mask;
}
/// \\brief  Enable PWM instance interrupt
///  \\ingroup hardware_pwm
///
/// Used to enable a single PWM instance interrupt
///
/// \\param slice_num PWM block to enable/disable
/// \\param enabled true to enable, false to disable
#[inline]
pub unsafe fn pwm_set_irq_enabled(slice_num: uint, enabled: bool) {
    if enabled {
        hw_set_bits(&mut (*pwm_hw).inte, 1 << slice_num);
    } else {
        hw_clear_bits(&mut (*pwm_hw).inte, 1 << slice_num);
    }
}
/// \\brief  Enable multiple PWM instance interrupts
///  \\ingroup hardware_pwm
///
/// Use this to enable multiple PWM interrupts at once.
///
/// \\param slice_mask Bitmask of all the blocks to enable/disable. Channel 0 = bit 0, channel 1 = bit 1 etc.
/// \\param enabled true to enable, false to disable
#[inline]
pub unsafe fn pwm_set_irq_mask_enabled(slice_mask: u32, enabled: bool) {
    if enabled {
        hw_set_bits(&mut (*pwm_hw).inte, slice_mask);
    } else {
        hw_clear_bits(&mut (*pwm_hw).inte, slice_mask);
    }
}
/// \\brief  Clear single PWM channel interrupt
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
#[inline]
pub unsafe fn pwm_clear_irq(slice_num: uint) {
    (*pwm_hw).intr = 1 << slice_num;
}
/// \\brief  Get PWM interrupt status, raw
///  \\ingroup hardware_pwm
///
/// \\return Bitmask of all PWM interrupts currently set
#[inline]
pub unsafe fn pwm_get_irq_status_mask() -> u32 {
    (*pwm_hw).ints
}
/// \\brief  Force PWM interrupt
///  \\ingroup hardware_pwm
///
/// \\param slice_num PWM slice number
#[inline]
pub unsafe fn pwm_force_irq(slice_num: uint) {
    (*pwm_hw).intf = 1 << slice_num;
}
