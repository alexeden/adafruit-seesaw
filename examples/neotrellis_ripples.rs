#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use core::ops::{Add, AddAssign};

/// https://github.com/adafruit/Adafruit_Seesaw/blob/master/examples/NeoTrellis/ripples/ripples.ino
use adafruit_seesaw::{devices::NeoTrellis, prelude::*, SeesawRefCell};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const MAX_RIPPLES: usize = 16;
const FALLOFF_TIME: usize = 64;
const NUM_POINTS: usize = 8;
const RIPPLE_RATE: f32 = 0.1;
const COLORS: [Color; 6] = [
    Color(255, 0, 0),
    Color(0, 255, 0),
    Color(0, 0, 255),
    Color(255, 255, 0),
    Color(0, 255, 255),
    Color(255, 0, 255),
];

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug, Default)]
struct Ripple {
    center: Point,
    t: usize,
    c: Color,
    points: [Point; NUM_POINTS],
}

#[derive(Copy, Clone, Debug, Default)]
struct ColorWheel(usize);

impl Iterator for ColorWheel {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = if self.0 >= COLORS.len() - 1 {
            0
        } else {
            self.0 + 1
        };
        Some(COLORS[self.0])
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Begin");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let seesaw = SeesawRefCell::new(delay, i2c);
    rprintln!("Seesaw created");
    let mut trellis = NeoTrellis::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start NeoTrellis");

    rprintln!("Trellis started");
    for x in 0..trellis.num_cols() {
        for y in 0..trellis.num_rows() {
            trellis
                .set_key_event_triggers(x, y, &[KeyEventType::Pressed], true)
                .and_then(|_| trellis.set_xy_neopixel_color(x, y, 0, 50, 0))
                .expect("Failed to set key events and neopixel color");
        }
    }

    trellis.sync_neopixel().expect("Failed to sync neopixel");

    let mut matrix: [[Color; 4]; 4] = [[Color::default(); 4]; 4];
    let mut ripples: [Option<Ripple>; MAX_RIPPLES] = [None; MAX_RIPPLES];
    let mut color_wheel = ColorWheel::default();

    rprintln!("Looping...");

    loop {
        // Clear the matrix
        for c in matrix.iter_mut().map(|x| x.iter_mut()).flatten() {
            *c = Color::default();
        }

        // Process events
        for event in trellis.read_key_events().expect("Failed to read events") {
            match event.event {
                KeyEventType::Pressed => {
                    for maybe_ripple in ripples.iter_mut() {
                        if let None = maybe_ripple {
                            let mut ripple = Ripple::default();
                            ripple.center = Point {
                                x: event.x as f32,
                                y: event.y as f32,
                            };
                            ripple.t = 0;
                            ripple.c = color_wheel.next().unwrap();
                            rprintln!("Ripple color: {:?}", ripple.c);
                            rprintln!("Ripple center: {:?}", ripple.center);
                            ripple.points = [ripple.center; NUM_POINTS];
                            *maybe_ripple = Some(ripple);
                            matrix[event.x as usize][event.y as usize] = ripple.c;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        let mut update = false;

        for maybe_ripple in ripples.iter_mut() {
            if let Some(ripple) = maybe_ripple {
                update = true;
                ripple.points[0].x += RIPPLE_RATE; // / 2.;
                ripple.points[1].x += RIPPLE_RATE; // / 2.;
                ripple.points[1].y += RIPPLE_RATE; // / 2.;
                ripple.points[2].y += RIPPLE_RATE;
                ripple.points[3].x -= RIPPLE_RATE; // / 2.;
                ripple.points[3].y += RIPPLE_RATE; // / 2.;
                ripple.points[4].x -= RIPPLE_RATE;
                ripple.points[5].x -= RIPPLE_RATE; // / 2.;
                ripple.points[5].y -= RIPPLE_RATE; // / 2.;
                ripple.points[6].y -= RIPPLE_RATE;
                ripple.points[7].x += RIPPLE_RATE; // / 2.;
                ripple.points[7].y -= RIPPLE_RATE; // / 2.;

                for p in ripple.points.iter() {
                    let x = p.x as i8;
                    let y = p.y as i8;
                    if x >= 0
                        && x < trellis.num_cols() as i8
                        && y >= 0
                        && y < trellis.num_rows() as i8
                    {
                        matrix[x as usize][y as usize] += ripple.c;
                    }
                }

                ripple.t += 1;

                if ripple.t > FALLOFF_TIME {
                    *maybe_ripple = None;
                }
            }
        }

        if update {
            let mut next_colors = [(0, 0, 0); 16];
            for (i, c) in matrix
                .iter_mut()
                .map(|x| x.iter_mut())
                .flatten()
                .enumerate()
            {
                next_colors[i] = (c.0, c.1, c.2);
            }
            // Update ripples
            trellis
                .set_neopixel_colors(&next_colors)
                .and_then(|_| trellis.sync_neopixel())
                .expect("Failed to set neopixel color");

            trellis.sync_neopixel().expect("Failed to sync neopixel");
        }
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

#[derive(Copy, Clone, Debug, Default)]
struct Color(pub u8, pub u8, pub u8);

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.0, value.1, value.2)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0.saturating_add(rhs.0),
            self.1.saturating_add(rhs.1),
            self.2.saturating_add(rhs.2),
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
