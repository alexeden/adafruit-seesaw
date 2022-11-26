use embedded_hal::blocking::{delay, i2c};

pub trait Bus<E>: delay::DelayUs<u32> {
    fn read<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<[u8; N], crate::SeesawError<E>>;

    fn write<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        bytes: &[u8; N],
    ) -> Result<(), crate::SeesawError<E>>
    where
        [(); N + 2]: Sized;

    fn read_u8(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<u8, crate::SeesawError<E>> {
        self.read::<1>(addr, reg).map(|buf| buf[0])
    }

    fn read_i32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<i32, crate::SeesawError<E>> {
        self.read::<4>(addr, reg).map(i32::from_be_bytes)
    }

    fn read_u16(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<u16, crate::SeesawError<E>> {
        self.read::<2>(addr, reg).map(u16::from_be_bytes)
    }

    fn read_u32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<u32, crate::SeesawError<E>> {
        self.read::<4>(addr, reg).map(u32::from_be_bytes)
    }

    fn write_u8(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        value: u8,
    ) -> Result<(), crate::SeesawError<E>> {
        self.write(addr, reg, &[value])
    }

    fn write_u16(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        value: u16,
    ) -> Result<(), crate::SeesawError<E>> {
        self.write(addr, reg, &u16::to_be_bytes(value))
    }

    fn write_i32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        value: i32,
    ) -> Result<(), crate::SeesawError<E>> {
        self.write(addr, reg, &i32::to_be_bytes(value))
    }

    fn write_u32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        value: u32,
    ) -> Result<(), crate::SeesawError<E>> {
        self.write(addr, reg, &u32::to_be_bytes(value))
    }
}

impl<BUS, E> Bus<E> for BUS
where
    BUS: delay::DelayUs<u32> + i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    fn write<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
        bytes: &[u8; N],
    ) -> Result<(), crate::SeesawError<E>>
    where
        [(); N + 2]: Sized,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0..2].copy_from_slice(reg);
        buffer[2..].copy_from_slice(bytes);

        self.write(addr, &buffer)
            .map(|_| self.delay_us(crate::DELAY_TIME))
            .map_err(crate::SeesawError::I2c)
    }

    fn read<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &crate::Reg,
    ) -> Result<[u8; N], crate::SeesawError<E>> {
        let mut buffer = [0u8; N];
        self.write(addr, reg)
            .and_then(|_| {
                self.delay_us(crate::DELAY_TIME);
                self.write_read(addr, &[], &mut buffer)
            })
            .map_err(crate::SeesawError::I2c)
            .map(|_| buffer)
    }
}
