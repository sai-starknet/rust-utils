#![no_std]
#![warn(missing_docs)]
//! Ergonomic conversion helpers for common standard library types.
//!
//! Provides extension traits that make it easy to convert inner types
//! using [`Into`] without verbose manual mapping.
//!
//! # Features
//! - `option` (default) — [`OptionInto`] for converting `Some` variants in-place
//! - `result` (default) — [`ResultInto`] for converting `Ok`/`Err` variants in-place
//! - `vec` (default) — [`VecInto`] for element-wise vector conversion
#[cfg(feature = "vec")]
extern crate alloc;

/// Conversion helpers for [`Option`] types.
#[cfg(feature = "option")]
pub mod option;
#[cfg(feature = "option")]
pub use option::OptionInto;

/// Conversion helpers for [`Result`] types.
#[cfg(feature = "result")]
pub mod result;
#[cfg(feature = "result")]
pub use result::ResultInto;

/// Conversion helpers for [`Vec`](alloc::vec::Vec) types.
#[cfg(feature = "vec")]
pub mod vec;
#[cfg(feature = "vec")]
pub use vec::VecInto;

/// Conversion helpers for [`HashMap`](hashbrown::HashMap) and [`HashSet`](hashbrown::HashSet) types.
#[cfg(feature = "hash")]
pub mod hash;
#[cfg(feature = "hash")]
pub use hash::{HashMapInto, HashSetInto};

/// Extension trait for converting the inner type of a container via [`Into`].
pub mod element;
pub use element::ElementInto;
