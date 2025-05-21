use crate::modules::Reg;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, Operation, SevenBitAddress},
};

const DELAY_TIME: u32 = 125;

/// Blanket trait for anything that implements I2C and a delay
pub trait Driver: I2c + DelayNs {}
impl<T> Driver for T where T: I2c + DelayNs {}

macro_rules! impl_integer_write {
    ($fn:ident $fn_with_delay:ident $nty:tt) => {
        fn $fn(
            &mut self,
            addr: SevenBitAddress,
            reg: &Reg,
            value: $nty,
        ) -> Result<(), Self::Error> {
            self.register_write(addr, reg, &<$nty>::to_be_bytes(value))
        }

        fn $fn_with_delay(
            &mut self,
            addr: SevenBitAddress,
            reg: &Reg,
            value: $nty,
            delay: u32,
        ) -> Result<(), Self::Error> {
            self.register_write_with_delay(addr, reg, &<$nty>::to_be_bytes(value), delay)
        }
    };
}

macro_rules! impl_integer_read {
    ($fn:ident $fn_with_delay:ident $nty:tt) => {
        fn $fn(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<$nty, Self::Error> {
            self.register_read::<{ ($nty::BITS / 8) as usize }>(addr, reg)
                .map($nty::from_be_bytes)
        }

        fn $fn_with_delay(&mut self, addr: SevenBitAddress, reg: &Reg, delay: u32) -> Result<$nty, Self::Error> {
            self.register_read_with_delay::<{ ($nty::BITS / 8) as usize }>(addr, reg, delay)
                .map($nty::from_be_bytes)
        }
    };
}

pub trait DriverExt {
    type Error;

    fn register_read_with_delay<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        delay: u32,
    ) -> Result<[u8; N], Self::Error>;

    fn register_write_with_delay(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8],
        delay: u32,
    ) -> Result<(), Self::Error>;

    impl_integer_read! { read_u8 read_u8_with_delay u8 }
    impl_integer_read! { read_u16 read_u16_with_delay u16 }
    impl_integer_read! { read_u32 read_u32_with_delay u32 }
    impl_integer_read! { read_u64 read_u64_with_delay u64 }
    impl_integer_read! { read_i8 read_i8_with_delay i8 }
    impl_integer_read! { read_i16 read_i16_with_delay i16 }
    impl_integer_read! { read_i32 read_i32_with_delay i32 }
    impl_integer_read! { read_i64 read_i64_with_delay i64 }
    impl_integer_write! { write_u8 write_u8_with_delay u8 }
    impl_integer_write! { write_u16 write_u16_with_delay u16 }
    impl_integer_write! { write_u32 write_u32_with_delay u32 }
    impl_integer_write! { write_u64 write_u64_with_delay u64 }
    impl_integer_write! { write_i8 write_i8_with_delay i8 }
    impl_integer_write! { write_i16 write_i16_with_delay i16 }
    impl_integer_write! { write_i32 write_i32_with_delay i32 }
    impl_integer_write! { write_i64 write_i64_with_delay i64 }

    fn register_read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], Self::Error> {
        self.register_read_with_delay(addr, reg, DELAY_TIME)
    }

    fn register_write(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        self.register_write_with_delay(addr, reg, bytes, DELAY_TIME)
    }
}

impl<T: Driver> DriverExt for T {
    type Error = T::Error;

    fn register_read_with_delay<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        delay: u32
    ) -> Result<[u8; N], Self::Error> {
        let mut buffer = [0u8; N];
        self.write(addr, reg)?;
        self.delay_us(delay);
        self.read(addr, &mut buffer)?;
        Ok(buffer)
    }

    fn register_write_with_delay(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8],
        delay: u32
    ) -> Result<(), Self::Error> {
        self.transaction(addr, &mut [Operation::Write(reg), Operation::Write(bytes)])?;
        self.delay_us(delay);
        Ok(())
    }
}
