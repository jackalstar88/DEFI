#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("Either feature `std` or `alloc` must be enabled for this crate.");
#[cfg(all(feature = "std", feature = "alloc"))]
compile_error!("Feature `std` and `alloc` can't be enabled at the same time.");

/// SBOR constants, like type id.
pub mod constants;
/// SBOR data model.
pub mod model;
/// Facade to Rust types.
pub mod rust;

mod decode;
mod describe;
mod encode;

pub use decode::{Decode, DecodeError, Decoder};
pub use describe::Describe;
pub use encode::{Encode, Encoder};

use crate::rust::vec::Vec;

/// Encode a `T` into byte array.
pub fn encode_with_metadata<T: Encode + ?Sized>(buf: Vec<u8>, v: &T) -> Vec<u8> {
    let mut enc = Encoder::with_metadata(buf);
    v.encode(&mut enc);
    enc.into()
}

/// Encode a `T` into byte array, with metadata stripped.
pub fn encode_no_metadata<T: Encode + ?Sized>(buf: Vec<u8>, v: &T) -> Vec<u8> {
    let mut enc = Encoder::no_metadata(buf);
    v.encode(&mut enc);
    enc.into()
}

/// Decode an instance of `T` from a slice.
pub fn decode_with_metadata<'de, T: Decode>(buf: &'de [u8]) -> Result<T, DecodeError> {
    let mut dec = Decoder::with_metadata(buf);
    let v = T::decode(&mut dec)?;
    dec.check_end()?;
    Ok(v)
}

/// Decode an instance of `T` from a slice which contains no metadata.
pub fn decode_no_metadata<'de, T: Decode>(buf: &'de [u8]) -> Result<T, DecodeError> {
    let mut dec = Decoder::no_metadata(buf);
    let v = T::decode(&mut dec)?;
    dec.check_end()?;
    Ok(v)
}

// Re-export derives
extern crate sbor_derive;
pub use sbor_derive::*;

// This is to make derives work within this crate.
// See: https://users.rust-lang.org/t/how-can-i-use-my-derive-macro-from-the-crate-that-declares-the-trait/60502
extern crate self as sbor;
