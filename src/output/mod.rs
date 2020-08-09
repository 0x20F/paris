//! Helper functions for writing to stdout/stderr
//!
//! Some can format, some cannot
use std::fmt::Display;

#[cfg(feature = "macros")]
use crate::formatter;

/// Gets the current timestamp or empty string
/// based on whether timestamps feature is enabled
fn current_time() -> String {
    #[cfg(feature = "timestamps")]
    {
        crate::timestamp::now()
    }

    #[cfg(not(feature = "timestamps"))]
    {
        String::new()
    }
}

/// Writes to stdout without replacing keys
#[cfg(not(feature = "no_logger"))]
pub fn stdout<T>(message: T, line_ending: &str)
where
    T: Display,
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", message);
}

/// Writes to stderr without replacing keys
#[cfg(not(feature = "no_logger"))]
pub fn stderr<T>(message: T, line_ending: &str)
where
    T: Display,
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", message);
}

/// Writes to stdout and replaces keys inside the given string
#[cfg(feature = "macros")]
pub fn format_stdout<T>(message: T, line_ending: &str)
where
    T: Display,
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", formatter::colorize_string(message));
}

/// Writes to stderr and replaces keys inside the given string
#[cfg(feature = "macros")]
pub fn format_stderr<T>(message: T, line_ending: &str)
where
    T: Display,
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", formatter::colorize_string(message));
}
