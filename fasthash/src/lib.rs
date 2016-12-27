//! A suite of non-cryptographic hash functions for Rust.
//!
//! # Example
//!
//! ```
//! use std::hash::{Hash, Hasher};
//!
//! use fasthash::{metro, MetroHasher};
//!
//! fn hash<T: Hash>(t: &T) -> u64 {
//!     let mut s = MetroHasher::new();
//!     t.hash(&mut s);
//!     s.finish()
//! }
//!
//! let h = metro::hash64(b"hello world\xff");
//!
//! assert_eq!(h, hash(&"hello world"));
//! ```
//!

extern crate extprim;
extern crate seahash;
extern crate fasthash_sys as ffi;

#[macro_use]
mod hasher;
pub mod city;
pub mod farm;
pub mod metro;
pub mod mum;
pub mod murmur;
pub mod murmur2;
pub mod murmur3;
pub mod sea;
pub mod spooky;
pub mod t1ha;
pub mod xx;

pub use hasher::{Fingerprint, FastHash, BufHasher, StreamHasher, HasherExt};

pub use city::CityHasher64 as CityHasher;
#[cfg(not(feature = "sse42"))]
pub use city::CityHasher128 as CityHasherExt;
#[cfg(feature = "sse42")]
pub use city::CityHashCrc128 as CityHasherExt;

pub use farm::{FarmHasher64 as FarmHasher, FarmHasher128 as FarmHasherExt};

#[cfg(not(feature = "sse42"))]
pub use metro::{MetroHasher64_1 as MetroHasher, MetroHasher128_1 as MetroHasherExt};
#[cfg(feature = "sse42")]
pub use metro::{MetroHasher64Crc_1 as MetroHasher, MetroHasher128Crc_1 as MetroHasherExt};

pub use mum::MumHasher;
pub use murmur::MurmurHasher;
pub use murmur2::Murmur2Hasher_x64_64 as Murmur2Hasher;
pub use murmur3::{Murmur3Hasher_x64_128 as Murmur3Hasher,
                  Murmur3Hasher_x64_128 as Murmur3HasherExt};
#[doc(no_inline)]
pub use sea::SeaHasher64 as SeaHasher;
pub use spooky::{SpookyHasher64 as SpookyHasher, SpookyHasher128 as SpookyHasherExt};

#[cfg(not(feature = "sse42"))]
pub use t1ha::T1ha64LeHasher as T1haHasher;
#[cfg(feature = "sse42")]
pub use t1ha::T1ha64CrcHasher as T1haHasher;

pub use xx::XXHasher64 as XXHasher;
