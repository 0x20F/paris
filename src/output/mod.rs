//!
//! TODO: This
//!
use std::fmt::Display;

#[cfg(feature = "macros")]
use crate::formatter;


fn current_time() -> String {
    #[cfg(feature = "timestamps")]
    {
        crate::timestamp::now()
    }

    #[cfg(not(eature = "timestamps"))]
    {
        String::new()
    }
}


#[cfg(not(feature = "no_logger"))]
pub fn stdout<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", message);
}

#[cfg(not(feature = "no_logger"))]
pub fn stderr<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", message);
}

#[cfg(feature = "macros")]
pub fn format_stdout<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", formatter::colorize_string(message));
}

#[cfg(feature = "macros")]
pub fn format_stderr<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", formatter::colorize_string(message));
}