#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{
    devices::{NeoKey1x4, NeoKey1x4Color, SoilSensor}, modules::touch::{self, TouchModule}, prelude::*, SeesawRefCell
};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const RED: NeoKey1x4Color = NeoKey1x4Color { r: 255, g: 0, b: 0 };
const GREEN: NeoKey1x4Color = NeoKey1x4Color { r: 0, g: 255, b: 0 };

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let mut delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
    let seesaw = SeesawRefCell::new(delay, i2c);
    let mut soil_sensor = SoilSensor::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start SoilSensor");

    loop {
        let touch_capacitance = soil_sensor.read_touch_capacitance();

        rprintln!("Current touch capacitance: {}", touch_capacitance);

        delay.delay_ms(1_000);
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
