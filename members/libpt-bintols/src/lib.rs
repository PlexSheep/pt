//* # Tools to work with binary values, memory, storage
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.

// official binary prefixes, see [https://en.wikipedia.org/wiki/Binary_prefix]
/// 2^10
pub const KIBI: usize = 2usize.pow(10);
/// 2^20
pub const MEBI: usize = 2usize.pow(20);
/// 2^30
pub const GIBI: usize = 2usize.pow(30);
/// 2^40
pub const TEBI: usize = 2usize.pow(40);
/// 2^50
pub const PEBI: usize = 2usize.pow(50);
/// 2^60
pub const EXBI: u128 = 2u128.pow(60);
// at this point, `usize` would overflow, so we have to switch to a bigger datatype.
/// 2^70
pub const ZEBI: u128 = 2u128.pow(70);
/// 2^80
pub const YOBI: u128 = 2u128.pow(80);

// use libpt_core;
pub mod datalayout;
pub mod display;
pub mod hedu;
