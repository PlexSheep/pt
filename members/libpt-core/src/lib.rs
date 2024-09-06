//! # common functionalities
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate implements core functionality useful for many use cases, such as macros,
//! formatting functions and more.

/// macros to make things faster in your code
pub mod macros;

/// ## Get the name of the crate that uses your library
///
/// Let's say you're writing the library `foo` and need the name of the crate that uses `foo`. With
/// this function, you can get the name of the crate that uses `foo`.
///
/// Will return [None] if [`std::env::current_exe()`] errors or if conversion to [String] from [std::ffi::OsStr] fails.
pub fn get_crate_name() -> Option<String> {
    if let Ok(exe) = std::env::current_exe() {
        return Some(exe.file_stem()?.to_str()?.to_string());
    }
    None
}
