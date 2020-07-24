extern crate chrono;

use chrono::{Timelike, Utc};

pub fn now() -> String {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();

    let stamp = format!(
        "<dimmed>{:02}:{:02}:{:02}.{:03} {}: </>",
        hour,
        now.minute(),
        now.second(),
        now.nanosecond() / 1_000_000,
        if is_pm { "PM" } else { "AM" }
    );

    stamp
}
