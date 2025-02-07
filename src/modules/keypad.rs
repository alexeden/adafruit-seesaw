use crate::{
    devices::SeesawDevice,
    driver::Driver,
    modules::{Modules, Reg},
    DriverExt, SeesawError,
};

#[allow(dead_code)]
const STATUS: &Reg = &[Modules::Keypad.into_u8(), 0x00];
const EVENT: &Reg = &[Modules::Keypad.into_u8(), 0x01];
const INT_SET: &Reg = &[Modules::Keypad.into_u8(), 0x02];
const INT_CLR: &Reg = &[Modules::Keypad.into_u8(), 0x03];
const COUNT: &Reg = &[Modules::Keypad.into_u8(), 0x04];
const FIFO: &Reg = &[Modules::Keypad.into_u8(), 0x10];

pub trait KeypadModule<D: Driver>: SeesawDevice<Driver = D> {
    const NUM_COLS: u8;
    const NUM_ROWS: u8;

    fn num_cols(&self) -> u8 {
        Self::NUM_COLS
    }

    fn num_rows(&self) -> u8 {
        Self::NUM_ROWS
    }

    fn disable_interrupt(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_CLR, 1)
            .map_err(SeesawError::I2c)
    }

    fn enable_interrupt(&mut self) -> Result<(), SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver()
            .write_u8(addr, INT_SET, 1)
            .map_err(SeesawError::I2c)
    }

    fn read_key_events(&mut self) -> Result<KeyEventIter, SeesawError<D::Error>> {
        let addr = self.addr();
        let n = self
            .driver()
            .read_u8(addr, COUNT)
            .map_err(SeesawError::I2c)?;
        if n == 0 {
            return Ok(KeyEventIter::default());
        }
        let events = self
            .driver()
            .register_read::<16>(addr, FIFO)
            .map_err(SeesawError::I2c)?;
        let mut buf: [Option<KeyEvent>; 16] = [None; 16];
        for i in 0..n as usize {
            buf[i] = Some(events[i].into());
        }
        Ok(KeyEventIter { buf, curr: 0 })
    }

    fn set_key_event_triggers(
        &mut self,
        x: u8,
        y: u8,
        types: &[KeyEventType],
        enable: bool,
    ) -> Result<(), SeesawError<D::Error>> {
        assert!(x < Self::NUM_COLS, "x greater than cols");
        assert!(y < Self::NUM_ROWS, "y greater than rows");
        let addr = self.addr();
        let key = (y << 3) + x;
        let edges = types.iter().fold(if enable { 1 } else { 0 }, |acc, e| {
            acc + (2_u8 << (*e as u8))
        });
        self.driver()
            .register_write(addr, EVENT, &[key, edges])
            .map_err(SeesawError::I2c)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct KeyEventIter {
    buf: [Option<KeyEvent>; 16],
    curr: usize,
}

impl Iterator for KeyEventIter {
    type Item = KeyEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.buf.len() {
            return None;
        }
        let rec = self.buf[self.curr];
        self.curr += 1;
        rec
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyEventType {
    /// steady-state key is down
    IsPressed = 0,
    /// steady-state key is up
    IsReleased = 1,
    /// one-shot as key is released
    Released = 2,
    /// one-shot as key is pressed
    Pressed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub event: KeyEventType,
    pub x: u8,
    pub y: u8,
}

impl From<u8> for KeyEvent {
    fn from(value: u8) -> Self {
        let event = match value & 3 {
            0 => KeyEventType::IsPressed,
            1 => KeyEventType::IsReleased,
            2 => KeyEventType::Released,
            3 => KeyEventType::Pressed,
            _ => unreachable!(),
        };
        let x = (value >> 2) & 7;
        let y = (value >> 2) >> 3;
        Self { event, x, y }
    }
}
