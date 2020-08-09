//!
//! TODO: This
//!
use std::fmt::Display;
use crate::formatter;


fn current_time() -> String {
    let mut timestamp = String::new();

    #[cfg(feature = "timestamps")]
    {
        timestamp = crate::timestamp::now();
    }

    timestamp
}


pub fn stdout<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", message);
}

pub fn stderr<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", message);
}

pub fn format_stdout<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    print!("{}", formatter::colorize_string(message));
}

pub fn format_stderr<T>(message: T, line_ending: &str)
    where
        T: Display
{
    let timestamp = current_time();
    let message = format!("{}{}{}", timestamp, message, line_ending);
    eprint!("{}", formatter::colorize_string(message));
}