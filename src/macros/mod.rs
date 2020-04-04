/// Default logging, this just outputs to the terminal
/// Use when you want full control of how you want
/// your log to look. All color keys work as normal.
///
/// # Example
/// ```
/// use paris::log;
///
/// log!("This <cyan>is <bright green>a log<//>!");
/// ```
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::output::stdout(format!($($arg)*), "\n");
    }
}


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
        $crate::output::stdout(format!("<cyan><info></> {}", format!($($arg)*)), "\n");
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
        $crate::output::stderr(format!("<red><cross></> {}", format!($($arg)*)), "\n");
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
        $crate::output::stdout(format!("<yellow><warn></> {}", format!($($arg)*)), "\n");
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
        $crate::output::stdout(format!("<green><tick></> {}", format!($($arg)*)), "\n");
    }
}






#[cfg(test)]
mod tests {
    #[test]
    fn macros() {
        log!("This <cyan>is <bright green>a log<//>!");
        info!("<red>HAHAHAHAHA<///> <black><on green>{}</>", "the crate supports macros with colors!");
        error!("This is going to <bright red>stderr</> {}", "WOOOO");
        warn!("This is a {} <yellow>BEWARE</>!", "warning");
        success!("{} went well, congrats!", "<bright green>Everything</>");


        match "a" {
            "a" => log!("It works inside a match as well!!! {}", "<bright blue>finally</>"),
            _ => unreachable!()
        }
    }
}