use super::Timestamp;
use std::fmt::Display;
use crate::output::Write;


pub struct Stderr {}

impl Timestamp for Stderr {}

impl Write for Stderr {
    fn write<T>(message: T, line_ending: &str) where
        T: Display
    {
        let timestamp = Self::current_time();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        eprint!("{}", message);
    }
}