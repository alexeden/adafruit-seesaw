#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{
    devices::{RotaryEncoder, RotaryEncoderColor},
    prelude::*,
    SeesawRefCell,
};
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
    let delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let seesaw = SeesawRefCell::new(delay, i2c);
    rprintln!("Seesaw created");
    let mut encoder = RotaryEncoder::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start RotaryEncoder");

    rprintln!(
        "Capabilities {:#?}",
        encoder.capabilities().expect("Failed to get options")
    );

    rprintln!("Looping...");
    let mut prev_position = 0;
    loop {
        let position = encoder.position(0).expect("Failed to get position");
        let c = color_wheel(((position & 0xFF) as u8).wrapping_mul(3));
        if position != prev_position {
            prev_position = position;
            rprintln!("Position changed to {}, new color is {:?}", position, c);
        }

        encoder
            .set_neopixel_color(c)
            .and_then(|_| encoder.sync_neopixel())
            .expect("Failed to set neopixel");

        if let Ok(true) = encoder.button(0) {
            rprintln!("Button pressed");
            encoder.set_position(0, 0).expect("Failed to set position");
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

fn color_wheel(byte: u8) -> RotaryEncoderColor {
    match byte {
        0..=84 => RotaryEncoderColor {
            r: 255 - byte * 3,
            g: 0,
            b: byte * 3,
        },
        85..=169 => RotaryEncoderColor {
            r: 0,
            g: (byte - 85) * 3,
            b: 255 - (byte - 85) * 3,
        },
        _ => RotaryEncoderColor {
            r: (byte - 170) * 3,
            g: 255 - (byte - 170) * 3,
            b: 0,
        },
    }
}
