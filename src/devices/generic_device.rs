// use super::{Connect, SeesawDevice};
// use crate::{
//     bus::{DelayBus, I2cBus},
//     modules::StatusModule,
//     SeesawBus, SeesawError,
// };
// use embedded_hal::blocking::i2c;
// use shared_bus::BusMutex;

// pub struct GenericDevice<M>(i2c::SevenBitAddress, M);

// impl<D, M> SeesawDevice<M::Bus, M> for GenericDevice<M>
// where
//     D: crate::Driver,
//     M: BusMutex<Bus = D>,
// {
//     fn addr(&self) -> u8 {
//         self.0.into()
//     }

//     fn bus<'a>(&'a self) -> &'a M {
//         &self.1
//     }
// }

// impl<D, M> StatusModule<M::Bus, M> for GenericDevice<M>
// where
//     D: crate::Driver,
//     M: BusMutex<Bus = D>,
// {
// }

// impl<I2C, DELAY> Connect<I2C, DELAY> for GenericDevice<SeesawBus<I2C, DELAY>>
// where
//     DELAY: DelayBus,
//     I2C: I2cBus,
// {
//     fn connect(
//         i2c: I2C,
//         delay: DELAY,
//         addr: i2c::SevenBitAddress,
//     ) -> Result<Self, SeesawError<I2C::I2cError>> {
//         let bus = SeesawBus::new(i2c, delay);
//         let device = Self(addr, bus);
//         // device.reset()
//         Ok(device)
//     }
// }
