//! Base 64
//!
//! This crate contains functions to encode and decode Base 64

#![cfg_attr(feature = "no-std", no_std)]

#[macro_use]
extern crate alloc;

mod prelude {
    pub use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
}
use prelude::*;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! unreachable {
    ( $($e:expr)? ) => {
        core::unreachable!($($e)?)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! unreachable {
    ( $($e:expr)? ) => {
        unsafe { core::hint::unreachable_unchecked() }
    };
}

mod decode;
pub use decode::decode;

mod encode;
pub use encode::encode;
#[cfg(not(feature = "no-std"))]
pub use encode::Base64Encoder;

#[cfg(feature = "bindings")]
mod bindings;

pub type Result<T> = core::result::Result<T, Cow<'static, str>>;
