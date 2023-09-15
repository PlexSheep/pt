/// # tests for the general behaviour of the libraries availability
///
/// These tests will not go very in depth

// IMPORTS /////////////////////////////////////////////////////////////////////////////////////////
use pt;

/// ## check if pt is loaded
#[test]
fn test_pt_is_loaded() {
    assert!(pt::is_loaded())
}
