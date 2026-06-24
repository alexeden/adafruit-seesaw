//! # Mini I2C Gamepad QT Logger for the Adafruit KB2040
//!
//! Logs joystick and button state over USB serial.

#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use adafruit_kb2040::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        fugit::RateExtU32,
        gpio::FunctionI2C,
        i2c::I2C,
        pac,
        timer::Timer,
        usb::UsbBus,
        watchdog::Watchdog,
        Sio,
    },
    XOSC_CRYSTAL_FREQ,
};
use adafruit_seesaw::{
    devices::{MiniI2cGamepadQt, MiniI2cGamepadQtState},
    prelude::*,
    SeesawDriver,
};
use core::fmt::{self, Write};
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator, descriptor::lang_id::LangID, device::StringDescriptors,
    prelude::*,
};
use usbd_serial::SerialPort;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = adafruit_kb2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        USB_BUS = Some(UsbBusAllocator::new(UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        )));
        USB_BUS.as_ref().unwrap()
    };

    let mut serial = SerialPort::new(usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x239A, 0x80F1))
        .strings(&[StringDescriptors::new(LangID::EN_US)
            .manufacturer("Adafruit")
            .product("KB2040 Mini I2C Gamepad QT Logger")
            .serial_number("0001")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let sda = pins.sda.into_function::<FunctionI2C>();
    let scl = pins.scl.into_function::<FunctionI2C>();
    let i2c = I2C::i2c0_with_external_pull_up(
        pac.I2C0,
        sda,
        scl,
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );
    let seesaw = SeesawDriver::new(timer, i2c);
    let mut gamepad = MiniI2cGamepadQt::new_with_default_addr(seesaw)
        .init()
        .expect("Failed to start Mini I2C Gamepad QT");

    let mut read_buf = [0u8; 64];
    let mut last_log = timer.get_counter_low();
    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            let _ = serial.read(&mut read_buf);
        }

        let now = timer.get_counter_low();
        if now.wrapping_sub(last_log) >= 100_000 {
            last_log = now;
            if let Ok(state) = gamepad.state() {
                log_state(&mut serial, state);
            }
        }
    }
}

fn log_state(serial: &mut SerialPort<UsbBus>, state: MiniI2cGamepadQtState) {
    let buttons = state.buttons;
    let _ = writeln!(
        UsbSerial(serial),
        "x={} y={} a={} b={} x_btn={} y_btn={} select={} start={}\r",
        state.x,
        state.y,
        buttons.a,
        buttons.b,
        buttons.x,
        buttons.y,
        buttons.select,
        buttons.start
    );
}

struct UsbSerial<'a, 'bus>(&'a mut SerialPort<'bus, UsbBus>);

impl Write for UsbSerial<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut bytes = s.as_bytes();
        while !bytes.is_empty() {
            match self.0.write(bytes) {
                Ok(count) => bytes = &bytes[count..],
                Err(UsbError::WouldBlock) => break,
                Err(_) => return Err(fmt::Error),
            }
        }
        Ok(())
    }
}
