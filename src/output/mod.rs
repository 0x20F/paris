//!
//!
//! Contains helper functions for outputting to
//! `stdout` and `stderr`.
//!
#[cfg(feature = "timestamps")]
use crate::timestamp;

use crate::formatter;
use std::fmt::Display;

/// Basically print! without all the argument formatting.
/// It does however replace keys with their respective color
/// codes and adds timestamps if feature is enabled.
pub fn stdout<T>(message: T, line_ending: &str)
where
    T: Display,
{
    #[cfg(feature = "timestamps")]
    {
        let timestamp = timestamp::now();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        print!("{}", formatter::colorize_string(message));
    }

    #[cfg(not(feature = "timestamps"))]
    {
        let message = format!("{}{}", message, line_ending);
        print!("{}", formatter::colorize_string(message));
    }
}

/// Basically eprint! without all the argument formatting.
/// It does however replace keys with their respective color
/// codes and adds timestamps if feature is enabled.
pub fn stderr<T>(message: T, line_ending: &str)
where
    T: Display,
{
    #[cfg(feature = "timestamps")]
    {
        let timestamp = timestamp::now();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        eprint!("{}", formatter::colorize_string(message));
    }

    #[cfg(not(feature = "timestamps"))]
    {
        let message = format!("{}{}", message, line_ending);
        eprint!("{}", formatter::colorize_string(message));
    }
}
