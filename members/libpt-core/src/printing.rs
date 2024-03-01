//! # tools that make printing stuff better

/// Quickly get a one line visual divider
pub fn divider() -> String {
    format!("{:=^80}", "=")
}

/// Quickly print a one line visual divider
pub fn print_divider() {
    println!("{:=^80}", "=")
}
