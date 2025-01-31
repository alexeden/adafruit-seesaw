#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
/// Arduino example: https://github.com/adafruit/Adafruit_Seesaw/blob/master/examples/NeoTrellis/ripples/ripples.ino
/// Demo video: https://storage.googleapis.com/apemedia/neotrellis576.mp4
use adafruit_seesaw::{devices::NeoTrellis, prelude::*, SeesawRefCell};
use core::ops::{Add, AddAssign, Mul};
use cortex_m_rt::entry;
use heapless::Deque;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

const MAX_RADIUS: f32 = 4.;
const RIPPLE_RATE: f32 = 0.15;
const RIPPLE_SPREAD: f32 = 1.;
const COLORS: [Color; 6] = [
    Color(255, 255, 0),
    Color(0, 255, 255),
    Color(255, 0, 255),
    Color(255, 0, 0),
    Color(0, 255, 0),
    Color(0, 0, 255),
];

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

    // Listen for key presses
    for x in 0..trellis.num_cols() {
        for y in 0..trellis.num_rows() {
            trellis
                .set_key_event_triggers(x, y, &[KeyEventType::Pressed], true)
                .expect("Failed to set key event triggers");
        }
    }

    let mut color_wheel = ColorWheel::default();
    let mut ripples: Deque<Ripple, 16> = Deque::new();
    let mut matrix: [Color; 16] = [Color::default(); 16];

    // Start a ripple on init
    ripples
        .push_back(Ripple::new(0., 0., color_wheel.next_color()))
        .unwrap();

    loop {
        matrix.fill(Color::default());

        // Process events
        for event in trellis.read_key_events().expect("Failed to read events") {
            #[allow(clippy::single_match)]
            match event.event {
                KeyEventType::Pressed => {
                    if ripples.is_full() {
                        ripples.pop_front().unwrap();
                    }
                    ripples
                        .push_back(Ripple::new(
                            event.x as f32,
                            event.y as f32,
                            color_wheel.next_color(),
                        ))
                        .unwrap();
                }
                _ => {}
            }
        }

        // Process ripples
        ripples
            .iter_mut()
            .filter(|r| r.radius <= MAX_RADIUS)
            .for_each(|ripple| {
                ripple.radius += RIPPLE_RATE;
                for (i, color) in matrix.iter_mut().enumerate() {
                    let dist = ripple.center.cheby_dist(&Point::new_from_index(i));
                    let z = RIPPLE_SPREAD - (ripple.radius - dist).abs();
                    *color += ripple.color * z;
                }
            });

        // Update neopixels
        trellis
            .set_neopixel_colors(&matrix.map(|c| c.into()))
            .and_then(|_| trellis.sync_neopixel())
            .expect("Failed to update neopixels");
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
    radius: f32,
    /// Color of the ripple
    color: Color,
}

impl Ripple {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            center: Point::new(x, y),
            radius: 0.,
            color,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct ColorWheel(usize);

impl ColorWheel {
    pub fn next_color(&mut self) -> Color {
        self.0 = if self.0 >= COLORS.len() - 1 {
            0
        } else {
            self.0 + 1
        };
        COLORS[self.0]
    }
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
