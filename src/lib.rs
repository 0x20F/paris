use std::fmt::{ Display, Formatter, Result };
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::io::prelude::*;
use std::io;

use chrono::{ Timelike, Utc };
use colored::*;



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
        let (mut t, mut c, mut i, mut w, mut h) = ("✔", "✖", "ℹ", "⚠", "♥"); 

        if cfg!(windows) {
            t = "√";
            c = "×";
            i = "i";
            w = "‼";
            h = "♥";
        }

        match *self {
            LogIcon::Tick       => write!(f, "{}", t),
            LogIcon::Cross      => write!(f, "{}", c),
            LogIcon::Info       => write!(f, "{}", i),
            LogIcon::Warning    => write!(f, "{}", w),
            LogIcon::Heart      => write!(f, "{}", h)
        }
    }
}






pub struct Logger {
    is_loading: Arc<AtomicBool>,
    loading_message: String, // TODO: Use Option<>
    loading_handle: Option<thread::JoinHandle<()>>,
    with_timestamp: bool,
}

impl Logger {
    /// Initializes a new logger
    /// 
    /// # Example
    /// ```
    /// use paris::Logger;
    /// let logger = Logger::new(true); // Passing true will add timestamps to all logs
    /// ```
    pub fn new(timestamp: bool) -> Logger {
        Logger {
            is_loading: Arc::new(AtomicBool::new(false)),
            loading_message: String::from(""),
            loading_handle: None,
            with_timestamp: if timestamp { true } else { false }
        }
    }



    /// Prints to stdout with no bells and whistles. I does however
    /// add a timestamp if enabled.
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// let logger = Logger::new(false);
    /// 
    /// logger.log("Basic and boring."); // Basic and boring.
    /// ```
    pub fn log<T: Display>(&self, message: T) -> &Logger {
        let timestamp = self.timestamp();

        println!("{}{}", timestamp, message);
        self
    }



    /// Prints to stdout and adds some info flair to the text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger.info("This is some info");
    /// ```
    pub fn info<T: Display>(&self, message: T) -> &Logger {
        let icon = format!("{}", LogIcon::Info);
        let timestamp = self.timestamp();

        println!("{} {}{}", icon.cyan(), timestamp, message);
        self
    }



    /// Prints to stdout and adds some success flair to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger.success("Everything went great!");
    /// ```
    pub fn success<T: Display>(&self, message: T) -> &Logger {
        let icon = format!("{}", LogIcon::Tick);
        let timestamp = self.timestamp();

        println!("{} {}{}", icon.green(), timestamp, message);
        self
    }



    /// Prints to stdout and adds some warning flare to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger.warning("This is a warning");
    /// ```
    pub fn warning<T: Display>(&self, message: T) -> &Logger {
        let icon = format!("{}", LogIcon::Warning);
        let timestamp = self.timestamp();

        println!("{} {}{}", icon.yellow(), timestamp, message);
        self
    }

    

    /// Prints to stderr and adds some error flare to text
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger.error("Something broke, here's the error");
    /// ```
    pub fn error<T: Display>(&self, message: T) -> &Logger {
        let icon = format!("{}", LogIcon::Cross);
        let timestamp = self.timestamp();

        eprintln!("{} {}{}", icon.red(), timestamp, message);
        self
    }



    /// Prints a specified amount of newlines to stdout
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger
    ///     .newline(5)
    ///     .info("Some newlines before info")
    ///     .newline(2)
    ///     .info("And some more in between");
    /// ```
    pub fn newline(&self, amount: usize) -> &Logger {
        print!("{}", "\n".repeat(amount));
        self
    }



    /// Prints a specified amount of tabs to stdout
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new(false);
    /// logger
    ///     .indent(1)
    ///     .warning("Indented warning eh? Stands out a bit")
    ///     .newline(5);
    /// ```
    pub fn indent(&self, amount: usize) -> &Logger {
        print!("{}", "\t".repeat(amount));
        self
    }



    /// Starts a loading animation with the given message. No other
    /// methods can be chained to this one.
    /// 
    /// # Example
    /// ```
    /// # use paris::Logger;
    /// let mut logger = Logger::new(false);
    /// logger.start_loading("Counting to 52!");
    /// 
    /// // counting happens here (somehow)
    /// 
    /// logger.stop_loading();
    /// logger.success("Done counting, only took 1 million years");
    /// ```
    pub fn start_loading<T: Display>(&mut self, message: T) {
        self.is_loading.store(true, Ordering::SeqCst);

        let status = self.is_loading.clone();
        let thread_message = message.to_string().clone();
        self.loading_message = message.to_string();

        self.loading_handle = Some(thread::spawn(move || {
            let frames: [&str; 6] = ["⠦", "⠇", "⠋", "⠙", "⠸", "⠴"];
            let mut i = 1;

            while status.load(Ordering::SeqCst) {
                if i == frames.len() {
                    i = 0;
                }

                print!("\r{} {}", frames[i].cyan(), thread_message);
                io::stdout().flush().unwrap();

                thread::sleep(Duration::from_millis(100));
                
                i = i + 1;
            }
        }));
    }



    /// Stops the loading animation and clears the line so you can print something else
    /// when loading is done, maybe a success message. No other methods can be chained
    /// to this one. 
    /// 
    /// Panics if `.start_loading()` wasn't called first.
    pub fn stop_loading(&mut self) {
        self.is_loading.store(false, Ordering::SeqCst);
        self.loading_handle
            .take().expect("Called stop on a non-existing thread, make sure you ran .start_loading() first!")
            .join().expect("Could not join spawned thread");

        let clearing_length = self.loading_message.len() + 5;
        print!("\r{}\r", " ".repeat(clearing_length));
        io::stdout().flush().unwrap();
    }





    /// Gets current timestamp in "00:00:00 AM/PM" format
    fn timestamp(&self) -> ColoredString {
        if !self.with_timestamp {
            return String::from("").normal();
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        format!(
            "{:02}:{:02}:{:02} {} > ", 
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" }
        ).bold()
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let logger = Logger::new(false);
        assert_eq!(logger.with_timestamp, false);
        logger.info("It doesn't have a timestamp");

        let logger = Logger::new(true);
        assert_eq!(logger.with_timestamp, true);
        logger.info("It has a timestamp");
    }

    #[test]
    fn test_loading() {
        let mut logger = Logger::new(false);
        logger.start_loading("Loading in the middle of a test is not good!");
        // Long thing here
        logger.stop_loading();
        logger.success("Done loading!");


        logger.info("About to load again");

        logger.start_loading("Loading something else");
        logger.stop_loading();
        
        logger
            .success("Done loading")
            .info("A bit after loading is finished");
    }

    #[test]
    #[should_panic]
    fn test_stop_loading() {
        let mut logger = Logger::new(false);
        // Stop loading without starting first
        logger.stop_loading();
    }

    #[test]
    fn it_works() {
        let logger = Logger::new(true);

        logger
            .info("Somebody")
            .error("Once")
            .warning("Told")
            .success("Me")
            .newline(5)
            .log("A basic log eh")
            .indent(2)
            .info("If it didn't crash it's fine");

        assert_eq!(logger.with_timestamp, true);
    }
}