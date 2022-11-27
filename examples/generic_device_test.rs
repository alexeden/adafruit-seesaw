#![no_std]
#![no_main]
use adafruit_seesaw::{devices::GenericDevice, SeesawBus, SeesawDevice};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::I2c,
    pac,
    prelude::*,
    rcc::{RccExt, SYSCLK_MAX},
};

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
    let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
    let bus = SeesawBus::new(i2c, delay);
    let _generic_device = GenericDevice::begin(bus, 0x30).expect("Failed to connect to a device.");

    loop {}
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

// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// #![no_std]
// #![no_main]
// use adafruit_seesaw::{
//     devices::{neokey_1x4::NeoKey1x4, SeesawDevice},
//     modules::neopixel::NeopixelModule,
//     SeesawBus,
// };
// use cortex_m_rt::entry;
// use rtt_target::{rprintln, rtt_init_print};
// use stm32f4xx_hal::{
//     gpio::GpioExt,
//     i2c::I2c,
//     pac,
//     prelude::*,
//     rcc::{RccExt, SYSCLK_MAX},
// };

// const RED: (u8, u8, u8) = (255, 0, 0);
// const GREEN: (u8, u8, u8) = (0, 255, 0);

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     rprintln!("Starting");
//     let cp = cortex_m::Peripherals::take().unwrap();
//     let dp = pac::Peripherals::take().unwrap();
//     let gpiob = dp.GPIOB.split();
//     let clocks = dp.RCC.constrain().cfgr.sysclk(SYSCLK_MAX.Hz()).freeze();
//     let delay = cp.SYST.delay(&clocks);
//     let scl = gpiob.pb6.into_alternate_open_drain::<4>();
//     let sda = gpiob.pb7.into_alternate_open_drain::<4>();
//     let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
//     let bus = SeesawBus::new(i2c, delay);
//     let mut neokeys = NeoKey1x4::begin_default(bus).expect("Failed to connect
// to neokeys");

//     loop {
//         let [k0, k1, k2, k3] = neokeys.keys_bool().expect("Failed to read
// keys");         neokeys
//             .set_neopixel_colors(&[
//                 if k0 { GREEN } else { RED },
//                 if k1 { GREEN } else { RED },
//                 if k2 { GREEN } else { RED },
//                 if k3 { GREEN } else { RED },
//             ])
//             .and_then(|_| neokeys.sync_neopixel())
//             .expect("Failed to update neopixels");
//     }
// }

// #[panic_handler]
// fn handle_panic(info: &core::panic::PanicInfo) -> ! {
//     rprintln!("PANIC! {}", info);
//     rprintln!("Location {:?}", info.location());
//     if let Some(pl) = info.payload().downcast_ref::<&str>() {
//         rprintln!("Payload {:?}", pl);
//     }
//     loop {}
// }
