extern crate chrono;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> String {
    let current = SystemTime::now();
    let since_epoch = current.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_epoch.as_secs();

    let hours = (timestamp % 86400 ) / 3600;
    let minutes = (timestamp % 3600) / 60;
    let seconds = timestamp % 60;

    let is_pm = hours > 12;

    let stamp = format!(
        "<dimmed>{:02}:{:02}:{:02} {}: </>",
        hours,
        minutes,
        seconds,
        if is_pm { "PM" } else { "AM" }
    );

    crate::formatter::colorize_string(stamp)
}
