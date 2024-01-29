use core::cell::RefCell;

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
impl<T> BusMutex for ::std::sync::Mutex<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        ::std::sync::Mutex::new(v)
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let mut v = self.lock().unwrap();
        f(&mut v)
    }
}
