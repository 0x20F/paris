#![macro_use]


#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        let message = format!("<cyan><info></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        let message = format!("<red><cross></> {}", format!($($arg)*));
        $crate::output::stderr(message, "\n");
    }
}

#[macro_export]
macro_rules! warn {
        ($($arg:tt)*) => {
        let message = format!("<yellow><warn></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        let message = format!("<green><tick></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}