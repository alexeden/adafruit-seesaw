#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{devices::NeoRotary4, prelude::*, SeesawRefCell};
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
    let mut encoder = NeoRotary4::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start NeoRotary4");
    rprintln!("Started");

    rprintln!("Looping...");
    let mut positions = [0i32; 4];

    loop {
        for (i, current_position) in positions.iter_mut().enumerate() {
            let position = encoder.position(i).expect("Failed to get position");
            if position != *current_position {
                *current_position = position;
                rprintln!("Position {} changed to {}", i, position);
            }
            let c = color_wheel(((position & 0xFF) as u8).wrapping_mul(3));
            let Color(r, g, b) = c.set_brightness(255);

            encoder
                .set_nth_neopixel_color(i, (r, g, b))
                .expect("Failed to set neopixel");

            if let Ok(true) = encoder.button(i) {
                rprintln!("Button {} pressed", i);
                encoder.set_position(i, 0).expect("Failed to set position");
            }
        }

        encoder.sync_neopixel().expect("Failed to sync neopixel");
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
