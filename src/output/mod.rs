//!
//! TODO: This
//!

mod stdout;
mod stderr;

pub use stdout::Stdout;
pub use stderr::Stderr;
use std::fmt::Display;


pub trait Timestamp {
    fn current_time() -> String {
        let mut timestamp = String::new();

        #[cfg(feature = "timestamps")]
        {
            timestamp = crate::timestamp::now();
        }

        timestamp
    }
}

pub trait Writer {
    fn write<T>(message: T, line_ending: &str)
        where
            T: Display;
}
