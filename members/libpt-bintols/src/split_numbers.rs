//! # Split numbers into bits and bytes
//!
//! Sometimes, you need a large integer in the form of many bytes, so split into [u8].
//! Rust provides

use num_traits::Unsigned;

/// split an integer into it's bytes, ignoring those bytes that would be all zero.
///
/// If the integer is zero, the Vec contains a single null byte.
pub fn unsigned_to_vec<T>(mut num: T) -> Vec<u8>
where
    T: Unsigned,
    T: std::ops::ShrAssign<i32>,
    T: std::cmp::PartialOrd<i32>,
    u8: std::convert::From<T>,
    T: Copy,
{
    if num == 0 {
        return vec![0];
    }
    let mut buf: Vec<u8> = Vec::new();
    while num > 0 {
        buf.push(num.into());
        num >>= 8;
    }
    buf
}
