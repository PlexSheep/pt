//! # A library for common needs
//!
//! `pt` aims to implement a number of functionalities that might me useful to develop
//! programs in Rust. It aims to be a collection of generally, possibly useful things.
//!
//! `pt` is a project consisting of multiple smaller crates, all bundled together in this
//! "main crate". Most crates will only show up if you activate their feature.

#[cfg(feature = "core")]
pub use pt_core     as core;
#[cfg(feature = "bintols")]
pub use pt_bintols  as bintols;
#[cfg(feature = "hedu")]
pub use pt_hedu     as hedu;
#[cfg(feature = "log")]
pub use pt_log      as log;
#[cfg(feature = "math")]
pub use pt_math     as math;
#[cfg(feature = "net")]
pub use pt_net      as net;
#[cfg(feature = "ccc")]
pub use pt_ccc      as ccc;
