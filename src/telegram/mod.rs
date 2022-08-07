//! Module providing utilities to be used in the [crate::bin::telegrambot].
//!
//! Remember while adding new features to this module that binaries [can only access the public API of the crate](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries).

pub mod display;
pub mod inline;
