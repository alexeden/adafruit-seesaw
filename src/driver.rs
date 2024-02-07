use crate::modules::Reg;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, SevenBitAddress},
};

const DELAY_TIME: u32 = 125;

/// Blanket trait for anything that implements I2C and a delay
pub trait Driver: I2c + DelayNs {}
impl<T> Driver for T where T: I2c + DelayNs {}

macro_rules! impl_integer_write {
    ($fn:ident $nty:tt) => {
        fn $fn(
            &mut self,
            addr: SevenBitAddress,
            reg: &Reg,
            value: $nty,
        ) -> Result<(), Self::Error> {
            self.register_write(addr, reg, &<$nty>::to_be_bytes(value))
        }
    };
}

macro_rules! impl_integer_read {
    ($fn:ident $nty:tt) => {
        fn $fn(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<$nty, Self::Error> {
            self.register_read::<{ ($nty::BITS / 8) as usize }>(addr, reg)
                .map($nty::from_be_bytes)
        }
    };
}

pub trait DriverExt {
    type Error;

    fn register_read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], Self::Error>;

    fn register_write<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized;

    impl_integer_read! { read_u8 u8 }
    impl_integer_read! { read_u16 u16 }
    impl_integer_read! { read_u32 u32 }
    impl_integer_read! { read_u64 u64 }
    impl_integer_read! { read_i8 i8 }
    impl_integer_read! { read_i16 i16 }
    impl_integer_read! { read_i32 i32 }
    impl_integer_read! { read_i64 i64 }
    impl_integer_write! { write_u8 u8 }
    impl_integer_write! { write_u16 u16 }
    impl_integer_write! { write_u32 u32 }
    impl_integer_write! { write_u64 u64 }
    impl_integer_write! { write_i8 i8 }
    impl_integer_write! { write_i16 i16 }
    impl_integer_write! { write_i32 i32 }
    impl_integer_write! { write_i64 i64 }
}

impl<T: Driver> DriverExt for T {
    type Error = T::Error;

    fn register_read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], Self::Error> {
        let mut buffer = [0u8; N];
        self.write(addr, reg)?;
        self.delay_us(DELAY_TIME);
        self.read(addr, &mut buffer)?;
        Ok(buffer)
    }

    fn register_write<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0..2].copy_from_slice(reg);
        buffer[2..].copy_from_slice(bytes);

        self.write(addr, &buffer)?;
        self.delay_us(DELAY_TIME);
        Ok(())
    }
}
