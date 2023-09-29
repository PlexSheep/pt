use libpt_bintols::*;

#[test]
fn mkdmp() {
    let v = vec![true, true, false];
    investigate_memory_layout!(bool, v);
}
