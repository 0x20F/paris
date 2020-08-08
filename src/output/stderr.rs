use super::Timestamp;
use std::fmt::Display;
use crate::output::Writer;


pub struct Stderr {}

impl Timestamp for Stderr {}

impl Writer for Stderr {
    fn write<T>(message: T, line_ending: &str) where
        T: Display
    {
        let timestamp = Self::current_time();
        let message = format!("{}{}{}", timestamp, message, line_ending);
        eprint!("{}", message);
    }
}