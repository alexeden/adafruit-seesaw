pub struct Reg<const M: u8, const F: u8>;

pub trait Register {
    fn module(&self) -> u8;
    fn function(&self) -> u8;
}

pub trait Writable: Register {}
pub trait Readable: Register {}
impl<const M: u8, const F: u8> Writable for Reg<M, F> {}
impl<const M: u8, const F: u8> Readable for Reg<M, F> {}
impl<const M: u8, const F: u8> Register for Reg<M, F> {
    fn module(&self) -> u8 {
        M
    }

    fn function(&self) -> u8 {
        F
    }
}
