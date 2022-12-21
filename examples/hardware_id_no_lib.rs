#![no_std]
#![no_main]
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const ADDR: u8 = 0x30;

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
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);

    // Reset device
    // i2c.write(ADDR, &[0x00, 0x7F, 0xFF])
    //     .map(|_| delay.delay_us(125_000u32))
    //     .expect("Failed to write to reset register");

    // Read hardware ID
    // let mut id_buf = [0x00u8];
    // i2c.write_read(ADDR, &[0x00, 0x01], &mut id_buf)
    //     .expect("Failed to read hardware ID register");

    let mut id_buf = [0x00u8];
    i2c.write(ADDR, &[0x00, 0x01])
        .map(|_| delay.delay_us(125u32))
        .and_then(|_| i2c.read(ADDR, &mut id_buf))
        .expect("Failed to read hardware ID register");
    // let mut id_buf = [0x00u8];
    // i2c.write(ADDR, &[0x00, 0x01])
    //     .map(|_| delay.delay_us(125u32))
    //     .and_then(|_| i2c.write_read(ADDR, &[0x00, 0x01], &mut id_buf))
    //     .expect("Failed to read hardware ID register");

    rprintln!("Hardware ID {:x?}", id_buf[0]);

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
