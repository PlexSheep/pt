//* # Tools that help display binary values, data sizes, etc

/// ## Get the binary representation for a Byte array [`&[u8]`]
///
/// ### Arguments
/// * `data` - The data you are trying to dump
pub fn bytes_to_bin(data: &[u8]) -> String {
    let mut s = format!("0b{:08b}", data.first().unwrap());
    for i in 1..data.len() {
        s.push_str(&format!("_{:08b}", data[i]));
        if i % 8 == 0 {
            s.push_str("\n")
        }
    }
    return s;
}

/// Quickly format a number of Bytes [`usize`] with the corresponding
/// number of bits
pub fn byte_bit_display(data: usize) -> String {
    format!("{:07} B = {:08} bit", data.clone(), data * 8)
}
