#[cfg(feature = "timestamps")]
use crate::timestamp;

use crate::formatter::Formatter;
use std::fmt::Display;




pub fn stdout<T>(message: T, line_ending: &str) where T: Display {
    #[cfg(feature = "timestamps")] {
        let timestamp = timestamp::now();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        print!("{}", Formatter::colorize_string(message));
    }

    #[cfg(not(feature = "timestamps"))] {
        let message = format!("{}{}", message, line_ending);
        print!("{}", Formatter::colorize_string(message));
    }
}



pub fn stderr<T>(message: T, line_ending: &str) where T: Display {
    #[cfg(feature = "timestamps")] {
        let timestamp = timestamp::now();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        eprint!("{}", Formatter::colorize_string(message));
    }

    #[cfg(not(feature = "timestamps"))] {
        let message = format!("{}{}", message, line_ending);
        eprint!("{}", Formatter::colorize_string(message));
    }
}


