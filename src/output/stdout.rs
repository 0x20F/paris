use super::Timestamp;
use std::fmt::Display;
use crate::output::Write;


pub struct Stdout {}

impl Timestamp for Stdout {}

impl Write for Stdout {
    fn write<T>(message: T, line_ending: &str) where
        T: Display
    {
        let timestamp = Self::current_time();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        print!("{}", message);
    }
}