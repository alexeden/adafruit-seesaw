use core::cell::RefCell;
use embedded_hal::{
    delay::DelayNs,
    i2c::{self, ErrorType, I2c},
};

/// Nearly-verbatim copy of the trait specified by the shared_bus crate
pub trait BusMutex {
    /// The actual bus that is wrapped inside this mutex.
    type Bus;

    /// Create a new mutex of this type.
    fn create(v: Self::Bus) -> Self;

    /// Lock the mutex and give a closure access to the bus inside.
    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R;
}

#[derive(Debug)]
pub struct RefCellBus<T>(RefCell<T>);

impl<T> BusMutex for RefCellBus<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        Self(RefCell::new(v))
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let bus = &mut *self.0.borrow_mut();
        f(bus)
    }
}

#[cfg(feature = "std")]
impl<T> BusMutex for std::sync::Mutex<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        std::sync::Mutex::new(v)
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let mut v = self.lock().unwrap();
        f(&mut v)
    }
}

#[derive(Debug)]
pub struct Bus<'a, M>(pub(crate) &'a M);

// Delay implementation
impl<DELAY, I2C, M> DelayNs for Bus<'_, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    fn delay_ns(&mut self, ns: u32) {
        self.0.lock(|bus| bus.0.delay_ns(ns))
    }
}

impl<DELAY, I2C, M> ErrorType for Bus<'_, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    type Error = I2C::Error;
}

impl<DELAY, I2C, M> I2c for Bus<'_, M>
where
    DELAY: DelayNs,
    I2C: I2c,
    M: BusMutex<Bus = (DELAY, I2C)>,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.0.lock(|bus| bus.1.transaction(address, operations))
    }
}
