/// # tests for the general behaviour of the libraries availability
///
/// These tests will not go very in depth

// IMPORTS /////////////////////////////////////////////////////////////////////////////////////////
use libpt;

/// ## check if libpt is loaded
#[test]
fn test_libpt_is_loaded() {
    assert!(libpt::is_loaded())
}
