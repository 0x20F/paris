#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fmt::{ Display, Formatter, Result };
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };
use std::io::prelude::*;
use std::io;

use chrono::{ Timelike, Utc };
use colored::*;
use std::borrow::Cow;
use regex::Regex;


/// Contains definitions for icons that can be
/// used in the terminal. See [this github repo](https://github.com/sindresorhus/figures) 
/// for an entire list. Use this in combination with printing macros.
pub enum LogIcon {
    /// A check mark, use when things go well
    /// 
    /// # Example
    /// ```
    /// # use paris::LogIcon;
    /// println!("{} Everything went well", LogIcon::Tick); 
    /// // ✔ Everything went well
    /// ```
    Tick,

    /// A cross, use when things go bad, or be creative
    /// 
    /// # Example
    /// ```
    /// # use paris::LogIcon;
    /// println!("{} Oops, try again!", LogIcon::Cross); 
    /// // ✖ Oops, try again!
    /// ```
    Cross,
    
    /// A fancy 'i', for information
    /// 
    /// # Example
    /// ```
    /// # use paris::LogIcon;
    /// println!("{} In Switzerland it is illegal to own just one guinea pig", LogIcon::Info); 
    /// // ℹ In Switzerland it is illegal to own just one guinea pig.
    /// ```
    Info,
    
    /// A triangle with an exclamation mark in it, dangerous
    /// 
    /// # Example
    /// ```
    /// # use paris::LogIcon;
    /// println!("{} Things are starting to catch fire!", LogIcon::Warning);
    /// // ⚠ Things are starting to catch fire!
    /// ```
    Warning,
    
    /// ❤️🦄
    /// # Example
    /// ```
    /// // You get it...
    /// ```
    Heart
}


impl Display for LogIcon {
    /// Match the enum value and print out the equivalent icon.
    /// On Windows, icons will be replaced with other *things* that
    /// are supported. See [this github repo](https://github.com/sindresorhus/figures) 
    /// for all replacements
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (
            mut tick,
            mut cross,
            mut info,
            mut warn,
            mut heart
        ) = ("✔", "✖", "ℹ", "⚠", "♥");

        if cfg!(windows) {
            tick = "√";
            cross = "×";
            info = "i";
            warn = "‼";
            heart = "♥";
        }

        match *self {
            LogIcon::Tick       => write!(f, "{}", tick),
            LogIcon::Cross      => write!(f, "{}", cross),
            LogIcon::Info       => write!(f, "{}", info),
            LogIcon::Warning    => write!(f, "{}", warn),
            LogIcon::Heart      => write!(f, "{}", heart)
        }
    }
}









pub struct Logger {
    is_loading: Arc<RwLock<bool>>,
    loading_message: String, // TODO: Use Option<>
    loading_handle: Option<thread::JoinHandle<()>>,
    with_timestamp: bool,
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
        // let icon = LogIcon::Info.to_string().cyan();
        self.stdout(format!("<info> {}", message))
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
        //let icon = LogIcon::Tick.to_string().green();
        self.stdout(format!("<tick> {}", message))
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
        //let icon = LogIcon::Warning.to_string().yellow();
        self.stdout(format!("<warn> {}", message))
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
        //let icon = LogIcon::Cross.to_string().red();
        self.stderr(format!("<cross> {}", message))
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

                print!("\r{} {}", frames[i].cyan(), Logger::parse_string(&thread_message));
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
        self
    }




    
    /// Gets current timestamp in "00:00:00 AM/PM" format
    fn timestamp(&self) -> ColoredString {
        if !self.with_timestamp {
            return String::from("").normal();
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        format!(
            "{:02}:{:02}:{:02}.{:03} {}: ",
            hour,
            now.minute(),
            now.second(),
            now.nanosecond() / 1_000_000,
            if is_pm { "PM" } else { "AM" }
        ).bold()
    }



    /// Output to stdout, add timestamps or on the same line
    fn stdout<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        let timestamp = self.timestamp();
        let message = Logger::parse_string(message.to_string());

        print!("{}{}{}", timestamp, message, self.get_line_ending());

        self
    }



    /// Output to stderr, add timestamps or write on the same line
    fn stderr<T>(&mut self, message: T) -> &mut Logger
        where T: Display
    {
        self.done();
        let timestamp = self.timestamp();
        let message = Logger::parse_string(message.to_string());

        eprint!("{}{}{}", timestamp, message, self.get_line_ending());

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



    fn parse_string<'a, S>(input: S) -> Cow<'a, str>
        where S: Into<Cow<'a, str>>
    {
        lazy_static!(
            static ref TAG: Regex =
                Regex::new(r"<(?:(?:[a-zA-Z]*+)|/(?:[a-zA-Z]*+))>")
                .unwrap();
        );

        let input = input.into();

        // Nothing to escape was found
        if TAG.find(&input).is_none() {
            return input;
        }


        let mut output = String::with_capacity(input.len());

        for mat in TAG.captures_iter(&input) {
            match &mat[0] {
                // Add colors above the icons

                "<info>" => output = input.replace(&mat[0], &LogIcon::Info.to_string()),
                "<cross>" => output = input.replace(&mat[0], &LogIcon::Cross.to_string()),
                "<tick>" => output = input.replace(&mat[0], &LogIcon::Tick.to_string()),
                "<warn>" => output = input.replace(&mat[0], &LogIcon::Warning.to_string()),
                _ => ()
            }
        }

        Cow::Owned(output)
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
        let s = String::from("</> <i> This is a string yooo <i> with icons");

        let parsed = Logger::parse_string(s);

        println!("parsed is '{}'", parsed);

        //assert!(!parsed.contains("<i>"));
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
