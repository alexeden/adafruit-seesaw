use crate::{
    common::{DeviceCapabilities, Modules, ProductDateCode, Reg},
    driver::Driver,
    DriverExt, SeesawDevice,
};

const STATUS_HW_ID: &Reg = &[Modules::Status.into(), 0x01];
const STATUS_VERSION: &Reg = &[Modules::Status.into(), 0x02];
const STATUS_OPTIONS: &Reg = &[Modules::Status.into(), 0x03];
const STATUS_TEMP: &Reg = &[Modules::Status.into(), 0x04];
const STATUS_SWRST: &Reg = &[Modules::Status.into(), 0x7F];

pub trait StatusModule<D: Driver>: SeesawDevice<D> {
    fn capabilities(&mut self) -> Result<DeviceCapabilities, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_OPTIONS)
            .map(|opts| opts.into())
            .map_err(crate::SeesawError::I2c)
    }

    fn hardware_id(&mut self) -> Result<u8, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();
        self.driver()
            .read_u8(addr, STATUS_HW_ID)
            .map_err(crate::SeesawError::I2c)
    }

    fn product_info(&mut self) -> Result<ProductDateCode, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_VERSION)
            .map(|version| version.into())
            .map_err(crate::SeesawError::I2c)
    }

    fn reset(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .write_u8(addr, STATUS_SWRST, 0xFF)
            .map(|_| self.driver().delay_us(125_000))
            .map_err(crate::SeesawError::I2c)
    }

    fn reset_and_verify_seesaw(&mut self) -> Result<(), crate::SeesawError<D::I2cError>> {
        let hw_id = Self::HARDWARE_ID;
        self.reset().and_then(|_| match self.hardware_id() {
            Ok(id) if id == hw_id => Ok(()),
            Ok(id) => Err(crate::SeesawError::InvalidHardwareId(id)),
            Err(e) => Err(e),
        })
    }

    fn temp(&mut self) -> Result<f32, crate::SeesawError<D::I2cError>> {
        let addr = self.addr();

        self.driver()
            .read_u32(addr, STATUS_TEMP)
            .map(|buf| (buf as f32 / (1u32 << 16) as f32))
            .map_err(crate::SeesawError::I2c)
    }
}
