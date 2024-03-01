use libpt_bintols::*;

#[test]
fn mkdmp() {
    let v = [true, true, false];
    investigate_memory_layout!(bool, v);
}
