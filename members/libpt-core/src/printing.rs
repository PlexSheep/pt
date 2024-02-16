//! # tools that make printing stuff better

// reimport our macros to this module, so the user does not get confused when importing the macros
pub use crate::divider;
pub use crate::print_divider;

/// Quickly get a one line visual divider
#[macro_export]
macro_rules! divider {
    () => {{
        format!("{:=^80}", "=")
    }};
}

/// Quickly print a one line visual divider
#[macro_export]
macro_rules! print_divider {
    () => {{
        println!("{}", divider!())
    }};
}
