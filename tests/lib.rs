/// # tests for the general behaviour of the libraries availability
///
/// These tests will not go very in depth

// IMPORTS /////////////////////////////////////////////////////////////////////////////////////////
use libpt;

#[test]
fn loaded_libpt() {
    assert!(libpt::libpt_loaded())
}
