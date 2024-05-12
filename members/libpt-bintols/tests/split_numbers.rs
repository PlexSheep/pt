use libpt_bintols::split_numbers::*;

#[test]
fn split_u128() {
    let source = [16, 255, 256, 0, u128::MAX, u64::MAX as u128];
    let correct = [
        vec![16],
        vec![255],
        vec![255, 1],
        vec![0],
        vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ],
        vec![255, 255, 255, 255, 255, 255, 255, 255],
    ];
    for (i, n) in source.iter().enumerate() {
        assert_eq!(unsigned_to_vec(*n), correct[i]);
    }
}

#[test]
fn split_u64() {
    let source = [16, 255, 256, 0, u64::MAX, u32::MAX as u64];
    let correct = [
        vec![16],
        vec![255],
        vec![1, 255],
        vec![0],
        vec![255, 255, 255, 255, 255, 255, 255, 255],
        vec![255, 255, 255, 255],
    ];
    for (i, n) in source.iter().enumerate() {
        assert_eq!(unsigned_to_vec(*n), correct[i]);
    }
}
