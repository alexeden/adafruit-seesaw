#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use adafruit_seesaw::{
    devices::{NeoSlider, RotaryEncoder},
    prelude::*,
    DirectI2cSeesaw,
};
use core::cell::RefCell;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Begin");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let i2c_ref_cell = RefCell::new(i2c);

    let encoder_delay = cp.SYST.delay(&clocks);
    let seesaw_encoder = DirectI2cSeesaw::new(
        encoder_delay,
        embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell),
    );
    let mut encoder = RotaryEncoder::new_with_default_addr(seesaw_encoder)
        .init()
        .expect("Failed to start RotaryEncoder");
    rprintln!(
        "Encoder Capabilities {:#?}",
        encoder.capabilities().expect("Failed to get options")
    );

    let neoslider_delay = dp.TIM2.delay_us(&clocks);
    let seesaw_neoslider = DirectI2cSeesaw::new(
        neoslider_delay,
        embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell),
    );
    let mut neoslider = NeoSlider::new_with_default_addr(seesaw_neoslider)
        .init()
        .expect("Failed to start RotaryEncoder");
    rprintln!(
        "Neoslider Capabilities {:#?}",
        neoslider.capabilities().expect("Failed to get options")
    );

    rprintln!("Looping...");
    let mut prev_encoder_position = 0;
    let mut prev_slider_value = 0;
    loop {
        let encoder_position = encoder.position(0).expect("Failed to get encoder position");
        if encoder_position != prev_encoder_position {
            prev_encoder_position = encoder_position;
            rprintln!("Position changed to {}", encoder_position);
        }
        let slider_value = neoslider.slider_value().expect("Failed to read slider");
        if slider_value != prev_slider_value {
            prev_slider_value = slider_value;
            rprintln!("Slider value changed to {}", slider_value);
        }
    }
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC! {}", info.message());
    if let Some(location) = info.location() {
        rprintln!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        rprintln!("Panic occurred but can't get location information...");
    }
    loop {}
}
