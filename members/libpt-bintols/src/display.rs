//* # Tools that help display binary values, data sizes, etc

use super::*;
pub use num_traits::{PrimInt, ToPrimitive};

/// ## Get the binary representation for a Byte array [`&[u8]`]
///
/// ### Arguments
/// * `data` - The data you are trying to dump
pub fn bytes_to_bin(data: &[u8]) -> String {
    let mut s = format!("0b{:08b}", data.first().unwrap());
    for dat in data {
        s.push_str(&format!("_{:08b}", dat));
        if dat % 8 == 0 {
            s.push('\n')
        }
    }
    s
}

/// Quickly format a number of Bytes [`usize`] with the corresponding
/// number of bits
pub fn byte_bit_display(data: usize) -> String {
    format!("{} B = {} bit", data.clone(), data * 8)
}

/// ## Format total byte sizes to human readable sizes
pub fn humanbytes<T>(total: T) -> String
where
    T: PrimInt,
    T: ToPrimitive,
    T: Ord,
    T: std::fmt::Display,
    T: std::fmt::Debug,
{
    if total < T::from(KIBI).unwrap() {
        return format!("{total} B");
    } else if T::from(KIBI).unwrap() <= total && total < T::from(MEBI).unwrap() {
        return format!("{:.2} K", total.to_f64().unwrap() / KIBI as f64);
    } else if T::from(MEBI).unwrap() <= total && total < T::from(GIBI).unwrap() {
        return format!("{:.2} M", total.to_f64().unwrap() / MEBI as f64);
    } else if T::from(GIBI).unwrap() <= total && total < T::from(TEBI).unwrap() {
        return format!("{:.2} G", total.to_f64().unwrap() / GIBI as f64);
    } else if T::from(TEBI).unwrap() <= total && total < T::from(PEBI).unwrap() {
        return format!("{:.2} T", total.to_f64().unwrap() / TEBI as f64);
    } else if T::from(PEBI).unwrap() <= total && total < T::from(EXBI).unwrap() {
        return format!("{:.2} P", total.to_f64().unwrap() / PEBI as f64);
    }
    // now we are starting to reach the sizes that are pretty unrealistic
    // (as of 2023 that is, hello future)
    //
    // the later ones overflow `usize` on 64 Bit computers, so we have
    // to work with a fixed, larger sized datatype
    else {
        let total: u128 = total.to_u128().unwrap();
        if EXBI <= total && total < ZEBI {
            return format!("{:.2} E", total.to_f64().unwrap() / EXBI as f64);
        } else if ZEBI <= total && total < YOBI {
            return format!("{:.2} Z", total.to_f64().unwrap() / ZEBI as f64);
        } else if YOBI <= total {
            return format!("{:.2} Y", total.to_f64().unwrap() / YOBI as f64);
        } else {
            unreachable!()
        }
    }
}
