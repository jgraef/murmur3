//! # Overview
//!
//! This crate provides an implementation of the [Murmur3] non-cryptographic
//! hash function.
//!
//! # **WORK IN PROGRESS**
//!
//! This crate is work-in-progress. Currently only the `x64_128` variant of
//! Murmur3 is implemented. There is a more mature version of murmur3 available [here](https://github.com/stusmall/murmur3).
//!
//! # Why a new crate?
//!
//! The before mentioned crate that already exists for Murmur3, only implements
//! it as a single function. Therefore it's impossible to create a hasher state
//! and update it in a streaming fashion. Furthermore this crate also provides
//! wrappers for [`std::io::Read`] and [`std::io::Write`] that compute a hash
//! function while data is being read or written.
//!
//! [Murmur3]: https://en.wikipedia.org/wiki/MurmurHash

#![feature(array_chunks)]

pub mod chunked;
pub mod hasher;
pub mod io;
pub mod seed;
pub mod state;

/// Compatibility to `murmur3` crate from crates.io
#[cfg(feature = "compat")]
mod compat;

pub use hasher::{Murmur3x64x128, hash};

#[cfg(feature = "compat")]
pub use compat::murmur3_x64_128;
