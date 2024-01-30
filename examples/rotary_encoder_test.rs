#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{devices::RotaryEncoder, prelude::*, SeesawRefCell};
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
    loop {
        let position = encoder.position().expect("Failed to get position");
        let c = color_wheel(((position & 0xFF) as u8).wrapping_mul(3));
        let Color(r, g, b) = c.set_brightness(255);

        encoder
            .set_neopixel_color(r, g, b)
            .and_then(|_| encoder.sync_neopixel())
            .expect("Failed to set neopixel");

        if let Ok(true) = encoder.button() {
            encoder.set_position(0).expect("Failed to set position");
        }
    }
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC! {}", info);
    rprintln!("Location {:?}", info.location());
    if let Some(pl) = info.payload().downcast_ref::<&str>() {
        rprintln!("Payload {:?}", pl);
    }
    loop {}
}

fn color_wheel(byte: u8) -> Color {
    match byte {
        0..=84 => Color(255 - byte * 3, 0, byte * 3),
        85..=169 => Color(0, (byte - 85) * 3, 255 - (byte - 85) * 3),
        _ => Color((byte - 170) * 3, 255 - (byte - 170) * 3, 0),
    }
}

struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn set_brightness(self, brightness: u8) -> Self {
        Self(
            ((self.0 as u16 * brightness as u16) >> 8) as u8,
            ((self.1 as u16 * brightness as u16) >> 8) as u8,
            ((self.2 as u16 * brightness as u16) >> 8) as u8,
        )
    }
}
