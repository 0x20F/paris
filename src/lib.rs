#[macro_use]
extern crate lazy_static;

pub mod icons;
pub mod colors;

use std::fmt::Display;
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };
use std::io::prelude::*;
use std::io;

use chrono::{ Timelike, Utc };
use colors::Parser;
use icons::LogIcon;





pub struct Logger {
    is_loading: Arc<RwLock<bool>>,
    loading_message: String, // TODO: Use Option<>
    loading_handle: Option<thread::JoinHandle<()>>,
    with_timestamp: bool,
    skip_timestamp: bool,
    line_ending: String
}

impl Logger {
    /// Initializes a new logger
    /// 
    /// # Example
    /// ```
    /// use paris::Logger;
    /// let logger = Logger::new(true); // Passing true will add timestamps to all logs
    /// ```
    pub fn new(include_timestamp: bool) -> Logger {
        Logger {
            is_loading      : Arc::new(RwLock::new(false)),
            loading_message : String::from(""),
            loading_handle  : None,

            with_timestamp  : include_timestamp,
            skip_timestamp  : false,

            line_ending     : String::from("\n")
        }
    }



    /// Prints to stdout with no bells and whistles. I does however
    /// add a timestamp if enabled.
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// let mut logger = Logger::new(false);
    /// 
    /// logger.log("Basic and boring."); // Basic and boring.
    /// ```
    pub fn log<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(message)
    }



    /// Prints to stdout and adds some info flair to the text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new(false);
    /// logger.info("This is some info");
    /// ```
    pub fn info<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<cyan>{}</> {}", LogIcon::Info, message))
    }



    /// Prints to stdout and adds some success flair to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new(false);
    /// logger.success("Everything went great!");
    /// ```
    pub fn success<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<green>{}</> {}", LogIcon::Tick, message))
    }



    /// Prints to stdout and adds some warning flare to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new(false);
    /// logger.warn("This is a warning");
    /// ```
    pub fn warn<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stdout(format!("<yellow>{}</> {}", LogIcon::Warning, message))
    }

    

    /// Prints to stderr and adds some error flare to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new(false);
    /// logger.error("Something broke, here's the error");
    /// ```
    pub fn error<T: Display>(&mut self, message: T) -> &mut Logger {
        self.stderr(format!("<red>{}</> {}", LogIcon::Cross, message))
    }



    /// Prints a specified amount of newlines to stdout
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let mut logger = Logger::new(false);
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
    /// # let mut logger = Logger::new(false);
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
    /// let mut logger = Logger::new(false);
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
    /// let mut logger = Logger::new(false);
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
        let thread_message = message.to_string();

        self.loading_message = message.to_string();

        self.loading_handle = Some(thread::spawn(move || {
            let frames: [&str; 6] = ["⠦", "⠇", "⠋", "⠙", "⠸", "⠴"];
            let mut i = 1;

            while *status.read().unwrap() {
                if i == frames.len() {
                    i = 0;
                }

                let message = format!(
                    "\r<cyan>{}</> {}",
                    frames[i],
                    &thread_message
                );

                print!("{}", Parser::parse_color_string(message));
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
            .take().expect("Called stop on a non-existing thread")
            .join().expect("Could not join spawned thread");

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
    /// # let mut logger = Logger::new(false);
    /// 
    /// logger
    ///     .same().log("This is on one line")
    ///     .indent(4)
    ///     .log("This is on the same line!");
    /// ```
    pub fn same(&mut self) -> &mut Logger {
        self.set_line_ending("");
        self.skip_timestamp();
        self
    }




    
    /// Gets current timestamp in "00:00:00 AM/PM" format
    fn timestamp(&mut self) -> String {
        if !self.with_timestamp || self.skip_timestamp {
            return String::from("");
            self.skip_timestamp = false;
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        let stamp = format!(
            "{:02}:{:02}:{:02}.{:03} {}: ",
            hour,
            now.minute(),
            now.second(),
            now.nanosecond() / 1_000_000,
            if is_pm { "PM" } else { "AM" }
        );

        stamp
    }



    /// Output to stdout, add timestamps or on the same line
    fn stdout<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        let timestamp = self.timestamp();
        let message = Parser::parse_color_string(message.to_string());

        print!("{}{}{}", timestamp, message, self.get_line_ending());

        self
    }



    /// Output to stderr, add timestamps or write on the same line
    fn stderr<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        let timestamp = self.timestamp();
        let message = Parser::parse_color_string(message.to_string());

        eprint!("{}{}{}", timestamp, message, self.get_line_ending());

        self
    }


    /// Toggle a flag
    fn skip_timestamp(&mut self) {
        self.skip_timestamp = true;
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

    #[test]
    fn timestamp() {
        let mut logger = Logger::new(false);
        assert_eq!(logger.with_timestamp, false);
        logger.info("It doesn't have a timestamp");

        let mut logger = Logger::new(true);
        assert_eq!(logger.with_timestamp, true);
        logger.info("It has a timestamp");
    }

    #[test]
    fn loading() {
        let mut logger = Logger::new(false);
        logger.loading("Loading in the middle of a test is not good!");
        // Long thing here
        logger.done().success("Done loading!");


        logger.info("About to load again");
        logger
            .loading("Loading something else")
            .done()
            .error("Done loading instantly lol");
    }

    #[test]
    fn same() {
        let mut logger = Logger::new(false);
        logger
            .same().success("This is on one line")
            .indent(1)
            .info("This is on the same line!!!")
            .error("But this one isn't");

        logger.same();
        assert_eq!(logger.line_ending, String::from(""));
    }

    #[test]
    fn parse() {
        let s = "<cyan>This <green>is <yellow>a <magenta>string<red> yooo</> with <blue>icons</>";

        let parsed = Parser::parse_color_string(s);

        println!("{}", parsed);

        assert!(!parsed.contains("<cyan>"));
        assert!(!parsed.contains("<yellow>"));
        assert!(!parsed.contains("<red>"));
        assert!(!parsed.contains("<blue>"));
        assert!(!parsed.contains("</>"));
    }

    #[test]
    fn it_works() {
        let mut logger = Logger::new(true);

        logger
            .info("Somebody")
            .error("Once")
            .warn("Told")
            .success("Me")
            .newline(5)
            .log("A basic log eh")
            .indent(2)
            .info("If it didn't crash it's fine");

        assert_eq!(logger.with_timestamp, true);
    }
}
