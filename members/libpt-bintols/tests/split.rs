use libpt_bintols::split::*;

#[test]
fn split_u128() {
    let source = [
        16,
        255,
        256,
        0,
        u128::MAX,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
    ];
    let correct = [
        vec![16],
        vec![255],
        vec![1, 0],
        vec![0],
        vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ],
        vec![255, 255, 255, 255, 255, 255, 255, 255],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    for (i, n) in source.iter().enumerate() {
        assert_eq!(unsigned_to_vec(*n), correct[i]);
    }
}

#[test]
fn split_u64() {
    let source = [
        16,
        255,
        256,
        0,
        u64::MAX,
        u32::MAX as u64,
        0b1_00000001,
        0b10011011_10110101_11110000_00110011,
    ];
    let correct = [
        vec![16],
        vec![255],
        vec![1, 0],
        vec![0],
        vec![255, 255, 255, 255, 255, 255, 255, 255],
        vec![255, 255, 255, 255],
        vec![1, 1],
        vec![0b10011011, 0b10110101, 0b11110000, 0b00110011],
    ];
    for (i, n) in source.iter().enumerate() {
        assert_eq!(unsigned_to_vec(*n), correct[i]);
    }
}
