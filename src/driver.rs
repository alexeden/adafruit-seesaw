use embedded_hal::blocking::{delay, i2c};

pub trait Driver: i2c::Write + i2c::WriteRead + i2c::Read + delay::DelayUs<u32> {
    type I2cError: From<<Self as i2c::Write>::Error>
        + From<<Self as i2c::WriteRead>::Error>
        + From<<Self as i2c::Read>::Error>;
}

impl<T, E> Driver for T
where
    T: i2c::Write<Error = E>
        + i2c::WriteRead<Error = E>
        + i2c::Read<Error = E>
        + delay::DelayUs<u32>,
{
    type I2cError = E;
}

pub trait DriverExt {
    type Error;

    fn register_read<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
    ) -> Result<[u8; N], Self::Error>;

    fn register_write<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized;

    fn read_u8(&mut self, addr: i2c::SevenBitAddress, reg: &[u8; 2]) -> Result<u8, Self::Error> {
        self.register_read::<1>(addr, reg).map(|buf| buf[0])
    }

    fn read_i32(&mut self, addr: i2c::SevenBitAddress, reg: &[u8; 2]) -> Result<i32, Self::Error> {
        self.register_read::<4>(addr, reg).map(i32::from_be_bytes)
    }

    fn read_u16(&mut self, addr: i2c::SevenBitAddress, reg: &[u8; 2]) -> Result<u16, Self::Error> {
        self.register_read::<2>(addr, reg).map(u16::from_be_bytes)
    }

    fn read_u32(&mut self, addr: i2c::SevenBitAddress, reg: &[u8; 2]) -> Result<u32, Self::Error> {
        self.register_read::<4>(addr, reg).map(u32::from_be_bytes)
    }

    fn write_u8(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        value: u8,
    ) -> Result<(), Self::Error> {
        self.register_write(addr, reg, &[value])
    }

    fn write_u16(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        value: u16,
    ) -> Result<(), Self::Error> {
        self.register_write(addr, reg, &u16::to_be_bytes(value))
    }

    fn write_i32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        value: i32,
    ) -> Result<(), Self::Error> {
        self.register_write(addr, reg, &i32::to_be_bytes(value))
    }

    fn write_u32(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        value: u32,
    ) -> Result<(), Self::Error> {
        self.register_write(addr, reg, &u32::to_be_bytes(value))
    }
}

impl<T: Driver> DriverExt for T {
    type Error = T::I2cError;

    fn register_read<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
    ) -> Result<[u8; N], Self::Error> {
        let mut buffer = [0u8; N];
        self.write(addr, reg)?;
        self.delay_us(crate::DELAY_TIME);
        self.write_read(addr, &[], &mut buffer)?;
        Ok(buffer)
    }

    fn register_write<const N: usize>(
        &mut self,
        addr: i2c::SevenBitAddress,
        reg: &[u8; 2],
        bytes: &[u8; N],
    ) -> Result<(), Self::Error>
    where
        [(); N + 2]: Sized,
    {
        let mut buffer = [0u8; N + 2];
        buffer[0..2].copy_from_slice(reg);
        buffer[2..].copy_from_slice(bytes);

        self.write(addr, &buffer)?;
        self.delay_us(crate::DELAY_TIME);
        Ok(())
    }
}
