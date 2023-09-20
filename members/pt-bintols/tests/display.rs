use pt_bintols::*;
use pt_bintols::display::*;

#[test]
fn btobin() {
    let data = [19, 19];
    let r = bytes_to_bin(&data);
    assert_eq!(r, format!("0b00010011_00010011"));
}

#[test]
fn big_btobin() {
    let data = [12,31,82,32,123,32,92,23,12,32,12,1,1,1];
    let r = bytes_to_bin(&data);
    assert_eq!(r, format!("0b00001100_00011111_01010010_\
    00100000_01111011_00100000_01011100_00010111_00001100\n\
    _00100000_00001100_00000001_00000001_00000001"));
}

#[test]
fn bybit() {
    assert_eq!(byte_bit_display(120), format!("120 B = 960 bit"));
    assert_eq!(byte_bit_display(12), format!("12 B = 96 bit"));
    assert_eq!(byte_bit_display(8), format!("8 B = 64 bit"));
}

#[test]
fn hmnbytes() {
    assert_eq!(humanbytes(0),       format!("0 B"));
    assert_eq!(humanbytes(1),       format!("1 B"));

    assert_eq!(humanbytes(KIBI-1),  format!("1023 B"));
    assert_eq!(humanbytes(KIBI),    format!("1.00 K"));
    assert_eq!(humanbytes(KIBI+1),  format!("1.00 K"));

    assert_eq!(humanbytes(MEBI-1),  format!("1024.00 K"));
    assert_eq!(humanbytes(MEBI),    format!("1.00 M"));
    assert_eq!(humanbytes(MEBI+1),  format!("1.00 M"));

    assert_eq!(humanbytes(GIBI-1),  format!("1024.00 M"));
    assert_eq!(humanbytes(GIBI),    format!("1.00 G"));
    assert_eq!(humanbytes(GIBI+1),  format!("1.00 G"));

    assert_eq!(humanbytes(TEBI-1),  format!("1024.00 G"));
    assert_eq!(humanbytes(TEBI),    format!("1.00 T"));
    assert_eq!(humanbytes(TEBI+1),  format!("1.00 T"));

    assert_eq!(humanbytes(PEBI-1),  format!("1024.00 T"));
    assert_eq!(humanbytes(PEBI),    format!("1.00 P"));
    assert_eq!(humanbytes(PEBI+1),  format!("1.00 P"));

    assert_eq!(humanbytes(EXBI-1),  format!("1024.00 P"));
    assert_eq!(humanbytes(EXBI),    format!("1.00 E"));
    assert_eq!(humanbytes(EXBI+1),  format!("1.00 E"));

    assert_eq!(humanbytes(ZEBI-1),  format!("1024.00 E"));
    assert_eq!(humanbytes(ZEBI),    format!("1.00 Z"));
    assert_eq!(humanbytes(ZEBI+1),  format!("1.00 Z"));

    assert_eq!(humanbytes(YOBI-1),  format!("1024.00 Z"));
    assert_eq!(humanbytes(YOBI),    format!("1.00 Y"));
    assert_eq!(humanbytes(YOBI+1),  format!("1.00 Y"));
}
