#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use core::ops::{Add, AddAssign, Mul};

/// https://github.com/adafruit/Adafruit_Seesaw/blob/master/examples/NeoTrellis/ripples/ripples.ino
use adafruit_seesaw::{devices::NeoTrellis, prelude::*, SeesawRefCell};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const FALLOFF_R: f32 = 4.;
const MAX_RIPPLES: usize = 16;
const RIPPLE_RATE: f32 = 0.2;
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

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_from_index(index: usize) -> Self {
        Self {
            x: (index % 4) as f32,
            y: (index / 4) as f32,
        }
    }

    pub fn cheby_dist(&self, other: &Self) -> f32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Ripple {
    center: Point,
    /// Radius from the ripple's center; incremented each loop by RIPPLE_RATE
    /// When it reaches FALLOFF_R, the ripple is dropped
    r: f32,
    /// Color of the ripple
    c: Color,
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

// const POINTS: [Point; 16] = {
//     let mut points = [Point::default(); 16];
//     for i in 0..4 {
//         for j in 0..4 {
//             points[i * 4 + j] = Point::new(i as f32, j as f32);
//         }
//     }
//     points
// };

//     Point::new(1.0, 0.0),
//     Point::new(1.0, 1.0),
//     Point::new(1.0, 2.0),
//     Point::new(1.0, 3.0),
//     Point::new(2.0, 0.0),
//     Point::new(2.0, 1.0),
//     Point::new(2.0, 2.0),
//     Point::new(2.0, 3.0),
//     Point::new(3.0, 0.0),
//     Point::new(3.0, 1.0),
//     Point::new(3.0, 2.0),
//     Point::new(3.0, 3.0),
// ];

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
                .and_then(|_| trellis.set_xy_neopixel_color(x, y, 10, 10, 10))
                .expect("Failed to set key events and neopixel color");
        }
    }

    trellis.sync_neopixel().expect("Failed to sync neopixel");

    let mut matrix: [[Color; 4]; 4] = [[Color::default(); 4]; 4];
    let mut ripples: [Option<Ripple>; MAX_RIPPLES] = [None; MAX_RIPPLES];
    let mut color_wheel = ColorWheel::default();

    rprintln!("Looping...");

    loop {
        // Process events
        for event in trellis.read_key_events().expect("Failed to read events") {
            match event.event {
                KeyEventType::Pressed => {
                    for maybe_ripple in ripples.iter_mut() {
                        if let None = maybe_ripple {
                            let mut ripple = Ripple::default();
                            ripple.center = Point::new(event.x as f32, event.y as f32);
                            ripple.r = 0.;
                            ripple.c = color_wheel.next().unwrap();
                            *maybe_ripple = Some(ripple);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        for y in 0..trellis.num_cols() {
            for x in 0..trellis.num_rows() {
                matrix[y as usize][x as usize] = Color::default();
                for maybe_ripple in ripples.iter() {
                    if let Some(ripple) = maybe_ripple {
                        let dist = ripple.center.cheby_dist(&Point::new(x as f32, y as f32));
                        let z = 1. - (ripple.r - dist).abs();
                        let color = ripple.c * z;
                        matrix[y as usize][x as usize] += color;
                    }
                }
            }
        }

        for maybe_ripple in ripples.iter_mut() {
            if let Some(ripple) = maybe_ripple {
                ripple.r += RIPPLE_RATE;

                if ripple.r > FALLOFF_R {
                    rprintln!("Ripple falling off");
                    *maybe_ripple = None;
                }
            }
        }

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

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(
            (self.0 as f32 * rhs) as u8,
            (self.1 as f32 * rhs) as u8,
            (self.2 as f32 * rhs) as u8,
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
