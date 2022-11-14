#![no_std]
#![no_main]
use adafruit_seesaw::{
    devices::{RotaryEncoder, SeesawDevice},
    modules::status::StatusModule,
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

const DEFAULT_ADDR: u8 = 0x36;

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
    let mut ss_bus = SeesawBus::new(i2c, delay);
    let encoder = RotaryEncoder::begin(&mut ss_bus, DEFAULT_ADDR)
        .expect("Failed to connect to rotary encoder");
    let temp = encoder.temp(&mut ss_bus).expect("Failed to get temp");
    rprintln!("Temp {:?}", temp);

    encoder.reset(&mut ss_bus).expect("Failed to reset device");
    let hardware_id = encoder
        .hardware_id(&mut ss_bus)
        .expect("Failed to get hardware ID");
    rprintln!("Hardware ID: {:?}", hardware_id);
    let version = encoder
        .product_info(&mut ss_bus)
        .expect("Failed to get version");
    rprintln!("Version {:?}", version);
    let temp = encoder.temp(&mut ss_bus).expect("Failed to get temp");
    rprintln!("Temp {:?}", temp);

    let options = encoder
        .capabilities(&mut ss_bus)
        .expect("Failed to get options");
    rprintln!("Options {:?}", options);

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
