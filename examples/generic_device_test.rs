#![no_std]
#![no_main]
use adafruit_seesaw::devices::{Connect, GenericDevice};
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
    let bus = shared_bus::BusManagerSimple::new(i2c);
    let _generic_device = GenericDevice::connect(bus.acquire_i2c(), delay, 0x30);
    // let _generic_device2 = GenericDevice::connect(bus.acquire_i2c(), delay,
    // 0x30);

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