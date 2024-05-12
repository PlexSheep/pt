//! # Split numbers into bits and bytes
//!
//! Sometimes, you need a large integer in the form of many bytes, so split into [u8].
//! Rust provides

/// split an integer into it's bytes, ignoring those bytes that would be all zero.
///
/// If the integer is zero, the Vec contains a single null byte.
pub fn unsigned_to_vec<T>(num: T) -> Vec<u8>
where
    u128: std::convert::From<T>
{
    let mut num: u128 = num.into();
    if num == 0 {
        return vec![0];
    }
    let mut buf: Vec<u8> = Vec::new();
    while num > 0 {
        buf.push(num as u8);
        num >>= 8;
    }
    buf
}
