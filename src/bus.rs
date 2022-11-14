use crate::error::SeesawError;
use embedded_hal::blocking::{delay::DelayUs, i2c::SevenBitAddress};

pub trait BusWrite<E>: DelayUs<u32> {
    fn write<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
        bytes: &[u8; N],
    ) -> Result<(), SeesawError<E>>
    where
        [(); N + 2]: Sized;

    fn write_u8(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
        value: u8,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, mod_reg, fn_reg, &[value])
    }

    fn write_u16(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
        value: u16,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, mod_reg, fn_reg, &u16::to_be_bytes(value))
    }

    fn write_i32(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
        value: i32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, mod_reg, fn_reg, &i32::to_be_bytes(value))
    }

    fn write_u32(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
        value: u32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, mod_reg, fn_reg, &u32::to_be_bytes(value))
    }
}

pub trait BusRead<E>: BusWrite<E> {
    fn read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
    ) -> Result<[u8; N], SeesawError<E>>;

    fn read_u8(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
    ) -> Result<u8, SeesawError<E>> {
        self.read::<1>(addr, mod_reg, fn_reg).map(|buf| buf[0])
    }

    fn read_i32(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
    ) -> Result<i32, SeesawError<E>> {
        self.read::<4>(addr, mod_reg, fn_reg)
            .map(i32::from_be_bytes)
    }

    fn read_u16(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
    ) -> Result<u16, SeesawError<E>> {
        self.read::<2>(addr, mod_reg, fn_reg)
            .map(u16::from_be_bytes)
    }

    fn read_u32(
        &mut self,
        addr: SevenBitAddress,
        mod_reg: u8,
        fn_reg: u8,
    ) -> Result<u32, SeesawError<E>> {
        self.read::<4>(addr, mod_reg, fn_reg)
            .map(u32::from_be_bytes)
    }
}
