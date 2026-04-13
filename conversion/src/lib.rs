#![no_std]
#![warn(missing_docs)]
//! Ergonomic conversion helpers for common standard library types.
//!
//! Provides extension traits that make it easy to convert inner types
//! using [`Into`] without verbose manual mapping.
//!
//! # Core traits (always available)
//! - [`OptionInto`] — convert the `Some` variant of an [`Option`] in-place
//! - [`ResultInto`] — convert the `Ok` and/or `Err` variants of a [`Result`] in-place
//!
//! # Feature flags
//! - **`alloc`** — enables [`VecInto`] and [`ItemsInto`] for element-wise collection conversion
//!   (requires the `alloc` crate)
//! - **`std`** (implies `alloc`) — enables [`HashMapInto`], [`HashSetInto`], and [`IntoHashMap`]
//!   for [`HashMap`](std::collections::HashMap) / [`HashSet`](std::collections::HashSet) conversion
//! - **`hash`** (default, implies `std`) — convenience flag that enables all hash-collection traits

/// Conversion helpers for wrapper types such as [`Option`](Option) and [`Result`](Result).
mod wrapper;
pub use wrapper::{OptionInto, ResultInto};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Conversion helpers for [`Vec`](alloc::vec::Vec) types.
#[cfg(feature = "alloc")]
mod allocs;
#[cfg(feature = "alloc")]
pub use allocs::{ItemsInto, VecInto};

/// Conversion helpers for [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet) types.
#[cfg(feature = "std")]
mod hash;
#[cfg(feature = "std")]
pub use hash::{HashMapInto, HashSetInto, IntoHashMap};
