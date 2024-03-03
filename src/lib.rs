//! # A library for common needs
//!
//! `pt` aims to implement a number of functionalities that might me useful to develop
//! programs in Rust. It aims to be a collection of generally, possibly useful things.
//!
//! `pt` is a project consisting of multiple smaller crates, all bundled together in this
//! "main crate". Most crates will only show up if you activate their feature.
#[cfg_attr(docsrs, doc(cfg(feature = "full")))]

#[cfg(feature = "bintols")]
pub use libpt_bintols as bintols;
#[cfg(feature = "core")]
pub use libpt_core as core;
#[cfg(feature = "log")]
pub use libpt_log as log;
