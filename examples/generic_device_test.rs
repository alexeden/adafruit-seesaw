#![no_std]
#![no_main]
use adafruit_seesaw::{prelude::*, SeesawBus, SeesawSingleThread};
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
    let mut seesaw_bus = SeesawBus::new(delay, bus.acquire_i2c());
    seesaw_bus
        .register_read::<0>(0, &[9, 0])
        .expect("Failed to read register");
    let _seesaw = SeesawSingleThread::new(seesaw_bus);
    // let _generic_device = GenericDevice::connect(bus.acquire_i2c(), delay, 0x30)
    //     .expect("Failed to connect generic device");
    // let _generic_device2 = GenericDevice::connect(bus.acquire_i2c(), delay,
    // 0x30);

    // rprintln!(
    //     "Product info {:#?}",
    //     generic_device
    //         .product_info()
    //         .expect("failed to get product info")
    // );
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
