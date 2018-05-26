//! This is a simple library for dealing with the CR, LF, CRLF problem.
//! It provides a trait `Enter`, And a default implement for `str`
//! that trim corresponding newline characters `ENTER` of running OS's stdin 

/// Definations of newline characters.  
pub(crate) mod newlines {
    #![allow(dead_code)]

    // MacOS now uses LF for newlines. Commented out.
    // const CR: &'static str = "\r";

    const LF: &'static str = "\n";
    const CRLF: &'static str = "\r\n";

    /// newlines' character is CRLF. This indicate you're using Windows OS.
    #[cfg(target_os = "windows")]
    pub const ENTER: &'static str = CRLF;

    /// newlines' character is LF. This indicate you're using non-Windows OS.
    #[cfg(not(target_os = "windows"))]
    pub const ENTER: &'static str = LF;
}

/// Constant representing newline characters in the target OS. 
/// The mapping of newline characters is as follows:
///
/// - Windows -> CRLF
/// - not Windows -> LF
pub use newlines::ENTER;

pub trait Enter {
    fn enter(&self) -> &str;
}

impl Enter for str {
    /// trimming corresponding newline characters of running OS's stdin.
    ///
    /// # example(s)
    /// 
    /// ```rust
    /// use enter::{ Enter, ENTER };
    /// 
    /// let mut str_with_enter = "Hello, World!".to_string();
    /// str_with_enter.push_str(ENTER);
    /// assert_eq!(str_with_enter.enter(), "Hello, World!");
    /// ```
    fn enter(&self) -> &str {
        self.trim_right_matches(ENTER)
    }
}
