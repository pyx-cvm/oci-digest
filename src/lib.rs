//! OCI Container Digests
//!
//! This crate provides types for working with OCI container image digests.
//!
//! # Features
//!
//! - `std`: Enables standard library integration
//! - `serde`: Enables serde serialization/deserialization
//! - `hasher`: Enables hashing functionality
//!
//! # Examples
//!
//! Basic digest parsing:
//!
//! ```rust
//! use oci_digest::Digest;
//!
//! let input = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
//!
//! match input.parse().unwrap() {
//!     Digest::Sha256(..) => (),
//!     _ => unreachable!(),
//! }
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code, clippy::expect_used, clippy::panic)]
#![deny(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "serde")]
extern crate alloc;

#[cfg(test)]
extern crate serde_json as _;

mod digest;
mod error;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "hasher")]
mod hasher;

#[cfg(all(feature = "hasher", feature = "std"))]
mod io;

pub use digest::Digest;
pub use error::Error;

#[cfg(feature = "hasher")]
#[cfg_attr(docsrs, doc(cfg(feature = "hasher")))]
pub use hasher::Hasher;

#[cfg(all(feature = "hasher", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "hasher", feature = "std"))))]
pub use io::{Measurable, Measurer, Verifier};
