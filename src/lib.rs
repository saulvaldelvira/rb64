//! Base 64
//!
//! This crate contains functions to encode and decode Base 64

#![cfg_attr(feature = "no-std", no_std)]

#[macro_use]
extern crate alloc;

mod prelude {
    pub use alloc::borrow::Cow;
    pub use alloc::string::String;
    pub use alloc::boxed::Box;
    pub use alloc::vec::Vec;
}
use prelude::*;

mod decode;
pub use decode::decode;
mod encode;
pub use encode::encode;

#[cfg(not(feature = "no-std"))]
mod reader;
#[cfg(not(feature = "no-std"))]
pub use reader::Base64Encoder;

#[cfg(feature = "bindings")]
mod bindings;

pub type Result<T> = core::result::Result<T,Cow<'static,str>>;

