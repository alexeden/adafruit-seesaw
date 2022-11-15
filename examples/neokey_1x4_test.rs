#![no_std]
#![no_main]
use adafruit_seesaw::{
    devices::{neokey_1x4::NeoKey1x4, SeesawDevice},
    SeesawBus,
};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::I2c,
    pac,
    prelude::*,
    rcc::{RccExt, SYSCLK_MAX},
};

const _DEFAULT_ADDR: u8 = 0x30;

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
    let mut bus = SeesawBus::new(i2c, delay);
    let neokeys = NeoKey1x4::begin_default(&mut bus).expect("Failed to connect to neokeys");
    // let encoder = RotaryEncoder(DEFAULT_ADDR);
    // let encoder2 = RotaryEncoder(DEFAULT_ADDR + 1);
    // encoder.reset(&mut ss_bus).expect("Failed to reset device");
    // let hardware_id = encoder
    //     .hardware_id(&mut ss_bus)
    //     .expect("Failed to get hardware ID");
    // rprintln!("Hardware ID: {:?}", hardware_id);
    // let hardware_id = encoder2
    //     .hardware_id(&mut ss_bus)
    //     .expect("Failed to get hardware ID");
    // rprintln!("Hardware ID: {:?}", hardware_id);
    // let version = encoder
    //     .product_info(&mut ss_bus)
    //     .expect("Failed to get version");
    // rprintln!("Version {:?}", version);
    // let options = encoder
    //     .capabilities(&mut ss_bus)
    //     .expect("Failed to get options");
    // rprintln!("Options {:?}", options);

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
