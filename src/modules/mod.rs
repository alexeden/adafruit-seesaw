pub trait SeesawModule {
    const MODULE_ID: u8;
}

pub trait Register<UX> {
    // type Ux;

    fn module() -> u8;
    fn function() -> u8;
}

pub struct HardwareId;

impl Register<u8> for HardwareId {
    fn module() -> u8 {
        0
    }

    fn function() -> u8 {
        1
    }
}
