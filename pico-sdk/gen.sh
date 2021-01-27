#!/bin/bash


bindgen wrapper.h \
    --use-core \
    --generate-inline-functions \
    --ctypes-prefix "crate::ctypes" \
    --disable-untagged-union \
    -- \
    -I $PICO_SDK_PATH/src/rp2_common/pico_stdio/include \
    -I $PICO_SDK_PATH/src/common/pico_stdlib/include \
    -I $PICO_SDK_PATH/src/common/pico_base/include \
    -I $PICO_SDK_PATH/src/common/pico_time/include \
    -I $PICO_SDK_PATH/src/rp2_common/pico_platform/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_base/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_timer/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_gpio/include \
    -I $PICO_SDK_PATH/src/rp2_common/hardware_uart/include \
    -I $PICO_SDK_PATH/src/rp2040/hardware_regs/include \
    -I $PICO_SDK_PATH/src/rp2040/hardware_structs/include \
    -I $PICO_SDK_PATH/src/boards/include \
    -I ./generated
