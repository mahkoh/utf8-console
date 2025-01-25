//! This crate provides a cross-platform function to enable UTF-8 console IO.

use std::io::Error;

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod imp;
#[cfg(test)]
mod tests;

/// Enables UTF-8 console IO.
///
/// This function does nothing on unix.
///
/// On windows, this function sets the input and output code page to UTF-8.
///
/// # Example
///
/// ```
/// utf8_console::enable().unwrap();
/// ```
pub fn enable() -> Result<(), Error> {
    imp::enable()
}
