//! A Rust implementation of the MetroHash algorithm.
//!
//! MetroHash is a high quality, high performance hash algorithm
//!
//! # Example
//!
//! ```rust
//! #![feature(hashmap_hasher)]
//!
//! use std::collections::HashMap;
//! use std::collections::hash_state::DefaultState;
//! use metrohash::MetroHash;
//!
//! let mut hash: HashMap<_, _, DefaultState<MetroHash>> = Default::default();
//! hash.insert(1000, "1000");
//! assert_eq!(hash.get(&1000), Some(&"1000"));
//! ```

#![cfg_attr(test, feature(test))]
mod metrohash64;
mod metrohash128;
mod utils;

pub use metrohash64::*;
pub use metrohash128::*;

pub type MetroHash = MetroHash64;

#[cfg(test)]
mod tests;
