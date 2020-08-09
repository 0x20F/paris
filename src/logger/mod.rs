use std::fmt::Display;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::formatter::Formatter;
use crate::output;

#[allow(missing_docs)]
pub struct Logger {
    is_loading: Arc<RwLock<bool>>,
    loading_message: String,
    loading_handle: Option<thread::JoinHandle<()>>,

    line_ending: String,
    formatter: Formatter,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            is_loading: Arc::new(RwLock::new(false)),
            loading_message: String::from(""),
            loading_handle: None,

            line_ending: String::from("\n"),
            formatter: Formatter::new(),
        }
    }
}

impl Logger {
    /// Initializes a new logger
    ///
    /// # Example
    /// ```
    /// use paris::Logger;
    /// let logger = Logger::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Prints to stdout with no bells and whistles. I does however
    /// add a timestamp if enabled.
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// let mut logger = Logger::new();
    ///
    /// logger.log("Basic and boring."); // Basic and boring.
    /// ```
    ///
    /// Equivalent macro: `log!()`
    pub fn log<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(message)
    }

    /// Prints to stdout and adds some info flair to the text
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger.info("This is some info");
    /// ```
    ///
    /// Equivalent macro: `info!()`
    pub fn info<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<cyan><info></> {}", message))
    }

    /// Prints to stdout and adds some success flair to text
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger.success("Everything went great!");
    /// ```
    ///
    /// Equivalent macro: `success!()`
    pub fn success<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<green><tick></> {}", message))
    }

    /// Prints to stdout and adds some warning flare to text
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger.warn("This is a warning");
    /// ```
    ///
    /// Equivalent macro: `warn!()`
    pub fn warn<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<yellow><warn></> {}", message))
    }

    /// Prints to stderr and adds some error flare to text
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger.error("Something broke, here's the error");
    /// ```
    ///
    /// Equivalent macro: `error!()`
    pub fn error<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stderr(format!("<red><cross></> {}", message))
    }

    /// Prints a specified amount of newlines to stdout
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger
    ///     .newline(5)
    ///     .info("Some newlines before info")
    ///     .newline(2)
    ///     .info("And some more in between");
    /// ```
    pub fn newline(&mut self, amount: usize) -> &mut Logger {
        self.done();
        print!("{}", "\n".repeat(amount));
        self
    }

    /// Prints a specified amount of tabs to stdout
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    /// logger
    ///     .indent(1)
    ///     .warn("Indented warning eh? Stands out a bit")
    ///     .newline(5);
    /// ```
    pub fn indent(&mut self, amount: usize) -> &mut Logger {
        self.done();
        print!("{}", "\t".repeat(amount));
        self
    }

    /// Starts a loading animation with the given message.
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// let mut logger = Logger::new();
    /// logger.loading("Counting to 52!");
    ///
    /// // counting happens here (somehow)
    ///
    /// logger
    ///     .done()
    ///     .success("Done counting, only took 1 million years");
    /// ```
    ///
    /// That's one way of doing it, but having to always call `.done()` doesn't
    /// look that tidy. Well you don't have to, unless you want. All other functions
    /// (success, info, error, etc.) call `.done()` just in case a loading thread is running
    /// already. A cleaner example would be:
    /// ```
    /// # use paris::Logger;
    /// let mut logger = Logger::new();
    /// logger.loading("Counting to 52! again");
    ///
    /// // ....
    ///
    /// logger.error("I give up, I can't do it again!");
    /// ```
    pub fn loading<T: Display>(&mut self, message: T) -> &mut Logger {
        let mut status = self.is_loading.write().unwrap();
        *status = true;

        drop(status); // Release the lock so a mutable can be returned

        let status = self.is_loading.clone();
        let message = self.formatter.colorize(&message.to_string());

        let thread_message = message.clone();
        self.loading_message = message;

        self.loading_handle = Some(thread::spawn(move || {
            let frames: [&str; 6] = ["⠦", "⠇", "⠋", "⠙", "⠸", "⠴"];
            let mut i = 1;

            while *status.read().unwrap() {
                if i == frames.len() {
                    i = 0;
                }

                let message = format!("\r<cyan>{}</> {}", frames[i], &thread_message);
                output::stdout(crate::formatter::colorize_string(message), "");
                io::stdout().flush().unwrap();

                thread::sleep(Duration::from_millis(100));

                i += 1;
            }
        }));

        self
    }

    /// Stops the loading animation and clears the line so you can print something else
    /// when loading is done, maybe a success message. All other methods (success, warning, error, etc.)
    /// call this one automatically when called so you can use one of those directly
    /// for less clutter.
    pub fn done(&mut self) -> &mut Logger {
        if !*self.is_loading.read().unwrap() {
            return self;
        }

        let mut status = self.is_loading.write().unwrap();
        *status = false;

        drop(status); // Release the lock so a mutable can be returned

        self.loading_handle
            .take()
            .expect("Called stop on a non-existing thread")
            .join()
            .expect("Could not join spawned thread");

        let clearing_length = self.loading_message.len() + 5;
        print!("\r{}\r", " ".repeat(clearing_length));
        io::stdout().flush().unwrap();

        self
    }

    /// Forces the next statement to not output a newline
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    ///
    /// logger
    ///     .same().log("This is on one line")
    ///     .indent(4)
    ///     .log("This is on the same line!");
    /// ```
    pub fn same(&mut self) -> &mut Logger {
        self.set_line_ending("");

        self
    }

    /// Add a custom key to the available list of keys
    ///
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new();
    ///
    /// logger.add_style("lol", vec!["green", "bold", "on_blue"]);
    ///
    /// // '<lol>' can now be used as a key in strings and will contain
    /// // the defined colors and styles
    /// logger.info("<lol>much shorter than writing all of them</>");
    pub fn add_style(&mut self, key: &str, colors: Vec<&str>) -> &mut Logger {
        self.formatter.new_style(key, colors);
        self
    }

    /// Output to stdout, add timestamps or on the same line
    fn stdout<T>(&mut self, message: T) -> &mut Logger
    where
        T: Display,
    {
        self.done();
        let message = message.to_string();

        output::stdout(self.formatter.colorize(&message), &self.get_line_ending());
        self
    }

    /// Output to stderr, add timestamps or write on the same line
    fn stderr<T>(&mut self, message: T) -> &mut Logger
    where
        T: Display,
    {
        self.done();
        let message = message.to_string();

        output::stderr(self.formatter.colorize(&message), &self.get_line_ending());
        self
    }

    /// Sets line ending to something specific
    /// mostly \n for now
    fn set_line_ending<T: Into<String>>(&mut self, ending: T) {
        self.line_ending = ending.into();
    }

    /// Return line ending based on whats already set
    /// set it back to newline if its not already
    fn get_line_ending(&mut self) -> String {
        let newline = String::from("\n");
        let empty = String::from("");

        if self.line_ending != newline {
            self.set_line_ending(newline);
            return empty;
        }

        newline
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn loading() {
        let mut logger = Logger::new();
        logger.loading("Loading in the middle of a test is not good!");
        thread::sleep(Duration::from_secs(1));
        logger.done().success("Done loading!");

        logger.info("About to load again");
        logger
            .loading("Loading something else")
            .done()
            .error("Done loading instantly lol");
    }

    #[test]
    fn same() {
        let mut logger = Logger::new();
        logger
            .same()
            .success("This is on one line")
            .indent(1)
            .info("This is on the same line!!!")
            .error("But this one isn't");

        logger.same();
        assert_eq!(logger.line_ending, String::from(""));

        logger.info("Reset the line");
        assert_eq!(logger.line_ending, String::from("\n"));
    }

    #[test]
    fn it_works() {
        let mut logger = Logger::new();

        logger
            .info("Somebody")
            .error("Once")
            .warn("Told")
            .success("Me")
            .newline(5)
            .log("A basic log eh")
            .indent(2)
            .info("If it didn't crash it's fine");
    }

    #[test]
    fn add_style_works() {
        let mut logger = Logger::new();

        logger.add_style("lmao", vec!["red", "on-green"]);
    }
}
