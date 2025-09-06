use crate::{devices::SeesawDevice, Driver, DriverExt, SeesawError};

use super::{Modules, Reg};

// https://github.com/adafruit/Adafruit_Seesaw/blob/master/Adafruit_seesaw.cpp#L363
// Delay of 3,000 us is used to read from the touch sensor. They also read the 
// value in a retry loop but I haven't seen that be necessary to get data.
const TOUCH_READ_DELAY: u32 = 3_000;

const TOUCH: &Reg = &[Modules::Touch.into_u8(), 0x10];

pub trait TouchModule<D: Driver>: SeesawDevice<Driver = D> {
    fn read_touch_capacitance(&mut self) -> Result<u16, SeesawError<D::Error>> {
        let addr = self.addr();
        self.driver().read_u16_with_delay(addr, TOUCH, TOUCH_READ_DELAY)
            .map_err(SeesawError::I2c)
    }
}
