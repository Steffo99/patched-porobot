//! Module providing utilities to be used in the `patched_porobot_telegram` executable target.
//!
//! While adding new features to this module, remember that binaries [can only access the public API of the crate](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries), as they considered a separate crate from the rest of the project.

pub mod display;
pub mod handler;
pub mod inline;
pub mod main;
