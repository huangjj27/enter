//! The only thing the crate doing is to privide constant CR, LF, CRLF and ENTER.

pub const CR: &'static str = "\r";
pub const LF: &'static str = "\n";
pub const CRLF: &'static str = "\r\n";

/// newlines' character is CRLF. This indicate you're using Windows OS.
#[cfg(target_os = "windows")]
pub const ENTER: &'static str = CRLF;

/// newlines' character is LF. This indicate you're using non-Windows OS.
#[cfg(not(target_os = "windows"))]
pub const ENTER: &'static str = LF;
