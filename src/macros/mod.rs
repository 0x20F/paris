#![macro_use]



macro_rules! info {
    ($($arg:tt)*) => {
        let message = format!("<cyan><info></> {}", format!($($arg)*));
        output::stdout(message, "\n");
    }
}


macro_rules! error {
    ($($arg:tt)*) => {
        let message = format!("<red><cross></> {}", format!($($arg)*));
        output::stderr(message, "\n");
    }
}


macro_rules! warn {
        ($($arg:tt)*) => {
        let message = format!("<yellow><warn></> {}", format!($($arg)*));
        output::stdout(message, "\n");
    }
}

macro_rules! success {
    ($($arg:tt)*) => {
        let message = format!("<green><tick></> {}", format!($($arg)*));
        output::stdout(message, "\n");
    }
}