#![no_std]
#![allow(incomplete_features, const_evaluatable_unchecked)]
#![feature(const_convert, const_trait_impl, generic_const_exprs)]
pub mod bus;
pub mod devices;
mod driver;
pub mod error;
mod modules;
pub(crate) use bus::*;
pub use error::SeesawError;
// use modules::Reg;

const DELAY_TIME: u32 = 125;
