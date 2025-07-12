#![no_std]
#![no_main]
use adafruit_seesaw::{
    devices::{NeoSlider, NeoSliderColor},
    prelude::*,
    SeesawDriver,
};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
    let seesaw = SeesawDriver::new(delay, i2c);
    let mut neoslider = NeoSlider::new_with_default_addr(seesaw)
        .init()
        .expect("Failed to start NeoSlider");

    let mut prev_color = color_wheel(0);
    loop {
        let value = neoslider.slider_value().expect("Failed to read slider");
        let color = color_wheel(((value / 3) & 0xFF) as u8);

        #[cfg(feature = "module_neopixel")]
        neoslider
            .set_neopixel_colors(&[color, color, color, color])
            .and_then(|_| neoslider.sync_neopixel())
            .expect("Failed to set neopixel colors");

        if color != prev_color {
            prev_color = color;
            rprintln!("Color changed to {:?}", color);
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

fn color_wheel(byte: u8) -> NeoSliderColor {
    match byte {
        0..=84 => NeoSliderColor {
            r: 255 - byte * 3,
            g: 0,
            b: byte * 3,
        },
        85..=169 => NeoSliderColor {
            r: 0,
            g: (byte - 85) * 3,
            b: 255 - (byte - 85) * 3,
        },
        _ => NeoSliderColor {
            r: (byte - 170) * 3,
            g: 255 - (byte - 170) * 3,
            b: 0,
        },
    }
}
