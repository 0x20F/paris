//! Simple way to output beautiful text in your
//! CLI applications. Only limit is your imagination.
//!
//! # How to use
//!
//!     use paris::Logger;
//!
//!     let mut log = Logger::new();
//!
//!     log.info("It's that simple!");
//!
//!  # Simple methods
//!
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     // You can have icons at the start of your message!
//!     log.info("Will add ℹ at the start");
//!     log.error("Will add ✖ at the start");
//!
//!  See [the `Logger` struct](./struct.Logger.html) for all methods
//!
//!
//! # Chaining
//! All methods can be chained together to build more intricate
//! log/message combinations, in hopes of minimizing the chaos
//! that every log string becomes when you have to concatenate
//! a bunch of strings and add tabs and newlines everywhere.
//!
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("this is some info")
//!        .indent(4).warn("this is now indented by 4")
//!        .newline(5)
//!        .success("and this is 5 lines under all other messages");
//!
//! # Customisation
//! Outputting text is cool. Outputting text with a colored icon
//! at the start is even cooler! But this crate is all about
//! customisation, about making the logs feel like home, if you will.
//! Included in the crate are a variety of keys you can use
//! to colorize your logs just the way you want them to be.
//!
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("I can write normal text or use tags to <red>color it</>");
//!     log.warn("Every function can contain <on green><black>tags</>");
//!
//!     log.info("If you don't write them <bleu>correctly</>, you just get an ugly looking tag");
//!
//! There's a key for all colors supported by the terminal `(white, black, red, blue, magenta, etc.)`
//! If you add the word `on` to any of those colors, it becomes the
//! background color instead `(on red, on blue, on green)`.
//!
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     // How useful...
//!     log.info("<on red> This has red background </>");
//!
//! Maybe you'd like to use your terminals brighter colors, if that's the case
//! you just have to add `bright` to your tag. Makes sense.
//!
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("<blue><on bright red> This text is blue on a bright red background</> it's a pain");
//!
//! See [the README](https://github.com/SirTheViking/logger/blob/master/README.md) for a full list of keys
//! if you're not feeling confident in your ability to name colors. It happens.
//!
//! ### Resetting
//! You've probably seen the `</>` tag in the above logs. It's not there to
//! _"close the previously opened tag"_ no no. You can open as many tags as you want
//! and only use `</>` once, it's just the _"reset everything to default"_ tag, You might
//! decide you don't ever want to use it. It's up to you.

//! However, resetting everything to default might not be what you want. Most of the time
//! it'll be enough, but for those times when it isn't there are a few other tags such as:

//! * `<///>` only resets the background
//! * `<//>` only reset the foreground
#![warn(missing_docs)]

#[cfg(feature = "timestamps")]
mod timestamp;

#[cfg(feature = "macros")]
pub mod macros;



mod formatter;
pub mod output;

use std::fmt::Display;
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };
use std::io::prelude::*;
use std::io;

pub use formatter::{ Formatter, LogIcon };










#[allow(missing_docs)]
pub struct Logger {
    is_loading: Arc<RwLock<bool>>,
    loading_message: String,
    loading_handle: Option<thread::JoinHandle<()>>,

    line_ending: String
}



impl Default for Logger {
    fn default() -> Self {
        Self {
            is_loading      : Arc::new(RwLock::new(false)),
            loading_message : String::from(""),
            loading_handle  : None,

            line_ending     : String::from("\n")
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

                print!("{}", Formatter::colorize_string(message));
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



    /// Output to stdout, add timestamps or on the same line
    fn stdout<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        output::stdout(message, &self.get_line_ending());
        self
    }



    /// Output to stderr, add timestamps or write on the same line
    fn stderr<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        output::stderr(message, &self.get_line_ending());
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


    #[cfg(feature = "macros")]
    #[test]
    fn macros() {
        info!("<red>HAHAHAHAHA<///> <black><on green>{}</>", "the crate supports macros with colors!");
        error!("This is going to <bright red>stderr</> {}", "WOOOO");
        warn!("This is a {} <yellow>BEWARE</>!", "warning");
        success!("{} went well, congrats!", "<bright green>Everything</>");
    }


    #[test]
    fn loading() {
        let mut logger = Logger::new();
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
        let mut logger = Logger::new();
        logger
            .same().success("This is on one line")
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
}
