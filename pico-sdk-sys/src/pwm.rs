use core::mem;

use crate::*;

// #define pwm_hw ((pwm_hw_t *const)PWM_BASE)
const pwm_hw: *mut pwm_hw_t = PWM_BASE as _;

#[inline]
pub unsafe fn pwm_gpio_to_slice_num(gpio: uint) -> uint {
    // static_assert!(gpio < N_GPIOS);
    (gpio >> 1) & 7
}

#[inline]
pub unsafe fn pwm_gpio_to_channel(gpio: uint) -> uint {
    gpio & 1
}

#[inline]
pub unsafe fn pwm_clear_irq(slice_num: uint) {
    (*pwm_hw).intr = 1 << slice_num;
}

pub unsafe fn pwm_set_irq_enabled(slice_num: uint, enabled: bool) {
    // valid_params_if(PWM, slice_num >= 0 && slice_num < NUM_PWM_SLICES);
    if enabled {
        hw_set_bits(&mut (*pwm_hw).inte, 1 << slice_num);
    } else {
        hw_clear_bits(&mut (*pwm_hw).inte, 1 << slice_num);
    }
}

#[inline]
unsafe fn pwm_config_set_output_polarity(c: &mut pwm_config, a: bool, b: bool) {
    c.csr = (c.csr & !(PWM_CH0_CSR_A_INV_BITS | PWM_CH0_CSR_B_INV_BITS))
        | (((a as u32) << PWM_CH0_CSR_A_INV_LSB) | ((b as u32) << PWM_CH0_CSR_B_INV_LSB));
}

#[inline]
pub unsafe fn pwm_config_set_clkdiv_int(c: &mut pwm_config, div: uint) {
    c.div = div << PWM_CH1_DIV_INT_LSB;
}

#[inline]
pub unsafe fn pwm_get_default_config() -> pwm_config {
    let mut c: pwm_config = mem::zeroed();
    // pwm_config_set_phase_correct(&c, false);
    let phase_correct = false;
    c.csr = (c.csr & !PWM_CH0_CSR_PH_CORRECT_BITS)
        | ((phase_correct as u32) << PWM_CH0_CSR_PH_CORRECT_LSB);

    pwm_config_set_clkdiv_int(&mut c, 1);
    // pwm_config_set_clkdiv_mode(&c, PWM_DIV_FREE_RUNNING);
    let mode = PWM_DIV_FREE_RUNNING;
    c.csr = (c.csr & !PWM_CH0_CSR_DIVMODE_BITS) | (mode << PWM_CH0_CSR_DIVMODE_LSB);

    pwm_config_set_output_polarity(&mut c, false, false);

    //pwm_config_set_wrap(&c, 0xffff);
    let wrap = 0xffff;
    c.top = wrap;
    c
}

#[inline]
pub unsafe fn pwm_init(slice_num: uint, c: &pwm_config, start: bool) {
    // valid_params_if(PWM, slice_num >= 0 && slice_num < NUM_PWM_SLICES);
    let slice_num = slice_num as usize;
    (*pwm_hw).slice[slice_num].csr = 0;
    (*pwm_hw).slice[slice_num].ctr = PWM_CH0_CTR_RESET;
    (*pwm_hw).slice[slice_num].cc = PWM_CH0_CC_RESET;
    (*pwm_hw).slice[slice_num].top = c.top;
    (*pwm_hw).slice[slice_num].div = c.div;
    (*pwm_hw).slice[slice_num].csr = c.csr | ((start as u32) << PWM_CH0_CSR_EN_LSB);
}

#[inline]
pub unsafe fn pwm_set_gpio_level(gpio: uint, level: uint16_t) {
    // valid_params_if(PWM, gpio < N_GPIOS);
    pwm_set_chan_level(
        pwm_gpio_to_slice_num(gpio),
        pwm_gpio_to_channel(gpio),
        level,
    );
}

pub unsafe fn pwm_set_chan_level(slice_num: uint, chan: uint, level: uint16_t) {
    // valid_params_if(PWM, slice_num >= 0 && slice_num < NUM_PWM_SLICES);
    let slice_num = slice_num as usize;
    hw_write_masked(
        &mut (*pwm_hw).slice[slice_num].cc,
        (level
            << (if chan != 0 {
                PWM_CH0_CC_B_LSB
            } else {
                PWM_CH0_CC_A_LSB
            })) as u32,
        if chan != 0 {
            PWM_CH0_CC_B_BITS
        } else {
            PWM_CH0_CC_A_BITS
        },
    );
}
