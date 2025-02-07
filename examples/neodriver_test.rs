#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{prelude::*, seesaw_device, Driver, SeesawRefCell};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::I2c,
    pac,
    prelude::*,
    rcc::{RccExt, SYSCLK_MAX},
};

seesaw_device! {
    name: NeoDriver,
    hardware_id: HardwareId::ATTINY817,
    product_id: 5766,
    default_addr: 0x60
}

const N_LEDS: usize = 50;

impl<D: Driver> NeopixelModule<D> for NeoDriver<D> {
    type Color = rgb::Grb<u8>;

    const N_LEDS: usize = N_LEDS;
    const PIN: u8 = 15;
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.sysclk(SYSCLK_MAX.Hz()).freeze();
    let delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let seesaw = SeesawRefCell::new(delay, i2c);
    let mut neo_driver = NeoDriver::new_with_default_addr(seesaw.acquire_driver());
    neo_driver
        .reset_and_verify_seesaw()
        .and_then(|_| neo_driver.enable_neopixel())
        .expect("Failed to init NeoDriver");

    let id = neo_driver.hardware_id().expect("Failed to get hardware id");
    rprintln!("Hardware ID {:?}", id);

    let mut colors: [rgb::Grb<u8>; N_LEDS] =
        core::array::from_fn(|i| color_wheel((i as u8).wrapping_mul(255 / N_LEDS as u8)));

    loop {
        colors.rotate_left(1);
        neo_driver
            .set_neopixel_colors(&colors)
            .and_then(|_| neo_driver.sync_neopixel())
            .expect("Failed to set neopixel colors");
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

const fn color_wheel(byte: u8) -> rgb::Grb<u8> {
    match byte {
        0..=84 => rgb::Grb {
            r: 255 - byte * 3,
            g: 0,
            b: byte * 3,
        },
        85..=169 => rgb::Grb {
            r: 0,
            g: (byte - 85) * 3,
            b: 255 - (byte - 85) * 3,
        },
        _ => rgb::Grb {
            r: (byte - 170) * 3,
            g: 255 - (byte - 170) * 3,
            b: 0,
        },
    }
}
