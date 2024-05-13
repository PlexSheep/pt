use libpt_bintols::join::*;

#[test]
fn join_u128() {
    let correct = [
        16,
        255,
        256,
        0,
        u128::MAX,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
    ];
    let source = [
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
        assert_eq!(array_to_unsigned::<u128>(n).unwrap(), correct[i]);
    }
}

#[test]
fn join_u64() {
    let correct = [
        16,
        255,
        256,
        0,
        u64::MAX,
        u32::MAX as u64,
        0b1_00000001,
        0b10011011_10110101_11110000_00110011,
    ];
    let source = [
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
        assert_eq!(array_to_unsigned::<u64>(n).unwrap(), correct[i]);
    }
}
