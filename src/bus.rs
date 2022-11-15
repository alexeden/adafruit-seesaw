use crate::{error::SeesawError, modules::Reg};
use embedded_hal::blocking::{delay::DelayUs, i2c::SevenBitAddress};

pub trait Bus<E>: DelayUs<u32> {
    fn read<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
    ) -> Result<[u8; N], SeesawError<E>>;

    fn write<const N: usize>(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        bytes: &[u8; N],
    ) -> Result<(), SeesawError<E>>
    where
        [(); N + 2]: Sized;

    fn read_u8(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<u8, SeesawError<E>> {
        self.read::<1>(addr, reg).map(|buf| buf[0])
    }

    fn read_i32(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<i32, SeesawError<E>> {
        self.read::<4>(addr, reg).map(i32::from_be_bytes)
    }

    fn read_u16(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<u16, SeesawError<E>> {
        self.read::<2>(addr, reg).map(u16::from_be_bytes)
    }

    fn read_u32(&mut self, addr: SevenBitAddress, reg: &Reg) -> Result<u32, SeesawError<E>> {
        self.read::<4>(addr, reg).map(u32::from_be_bytes)
    }

    fn write_u8(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        value: u8,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &[value])
    }

    fn write_u16(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        value: u16,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &u16::to_be_bytes(value))
    }

    fn write_i32(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        value: i32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &i32::to_be_bytes(value))
    }

    fn write_u32(
        &mut self,
        addr: SevenBitAddress,
        reg: &Reg,
        value: u32,
    ) -> Result<(), SeesawError<E>> {
        self.write(addr, reg, &u32::to_be_bytes(value))
    }
}
