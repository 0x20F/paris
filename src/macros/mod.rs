#![macro_use]


/// Adds an info icon to the log message,
/// then writes to `stdout`.
///
/// # Example
/// ```
/// use paris::info;
///
/// info!("This is some info");
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        let message = format!("<cyan><info></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}


/// Adds an error icon to the log message,
/// then writes to `stderr`.
///
/// # Example
/// ```
/// use paris::error;
///
/// error!("Everything is burning");
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        let message = format!("<red><cross></> {}", format!($($arg)*));
        $crate::output::stderr(message, "\n");
    }
}


/// Adds a warning icon to the log message,
/// then writes to `stdout`.
///
/// # Example
/// ```
/// use paris::warn;
///
/// warn!("Everything could start burning!");
/// ```
#[macro_export]
macro_rules! warn {
        ($($arg:tt)*) => {
        let message = format!("<yellow><warn></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}


/// Adds a success icon to the log message,
/// then writes to `stdout`.
///
/// # Example
/// ```
/// use paris::success;
///
/// success!("You did it!");
/// ```
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        let message = format!("<green><tick></> {}", format!($($arg)*));
        $crate::output::stdout(message, "\n");
    }
}