#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{devices::NeoKey1x4, prelude::*, SeesawRefCell};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const RED: (u8, u8, u8) = (255, 0, 0);
const GREEN: (u8, u8, u8) = (0, 255, 0);

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
    let seesaw = SeesawRefCell::new(delay, i2c);
    let mut neokeys = NeoKey1x4::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start NeoKey1x4");

    loop {
        let keys = neokeys.keys().expect("Failed to read keys");

        neokeys
            .set_neopixel_colors(&[
                if (keys >> 0) & 1 == 0 { GREEN } else { RED },
                if (keys >> 1) & 1 == 0 { GREEN } else { RED },
                if (keys >> 2) & 1 == 0 { GREEN } else { RED },
                if (keys >> 3) & 1 == 0 { GREEN } else { RED },
            ])
            .and_then(|_| neokeys.sync_neopixel())
            .expect("Failed to update neopixels");
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
