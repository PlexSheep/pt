//! # Join bits and bytes into numbers
//!
//! Sometimes you have a `[u8]` that is the representation of a larger unsigned integer, such as
//! [u128]. This module helps you join them together.

use anyhow::anyhow;
use libpt_log::trace;

/// Join a [Vec] of [u8]s into an unsigned integer
///
/// Say you have the array `[0b00000110, 0b10110101]` and want to use it as a [u32].
/// This function sets it together to a integer type of your choosing:
/// 1717 (binary: `00000000 00000000 00000110 10110101`).
///
/// If the array is not long enough, the number will be padded with null bytes.
///
/// # Examples
///
/// ```
/// # use libpt_bintols::join::*;
///
/// let x: [u8; 2] = [0b00000110, 0b10110101];
///
/// assert_eq!(array_to_unsigned::<u32>(&x).unwrap(), 1717);
/// ```
pub fn array_to_unsigned<T>(parts: &[u8]) -> anyhow::Result<T>
where
    u128: std::convert::From<T>,
    T: std::str::FromStr,
    T: std::convert::TryFrom<u128>,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Send,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Sync,
    <T as std::convert::TryFrom<u128>>::Error: 'static,
{
    trace!("amount of parts: {}", parts.len());
    if parts.len() > (u128::BITS / 8) as usize {
        return Err(anyhow!(
            "the list is too long to fit into the specified integer type: {}",
            std::any::type_name::<T>()
        ));
    }
    let mut ri: u128 = 0;
    for (i, e) in parts.iter().rev().enumerate() {
        ri += (*e as u128) * 256u128.pow(i as u32);
    }
    T::try_from(ri).map_err(anyhow::Error::from)
}
