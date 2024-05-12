//! # Split numbers into bits and bytes
//!
//! Sometimes, you need a large integer in the form of many bytes, so split into [u8].
//! Rust provides

/// Split unsigned integers into a [Vec] of [u8]s
///
/// Say you have the [u32] 1717 (binary: `00000000 00000000 00000110 10110101 `). This number would
/// be splitted to `vec![0b00000110, 0b10110101]`.
///
/// The 0 bytes of the numbers will be discarded (unless the number is 0, then the Vec contains a
/// single Null byte.) and the remaining parts of the numbers are inserted into a Vec as [u8].
///
/// # Examples
///
/// ```
/// # use libpt_bintols::split_numbers::*;
///
/// let x: u32 = 1717;
///
/// assert_eq!(unsigned_to_vec(x), vec![0b00000110, 0b10110101]);
/// ```
pub fn unsigned_to_vec<T>(num: T) -> Vec<u8>
where
    u128: std::convert::From<T>,
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
    buf.reverse();
    buf
}
