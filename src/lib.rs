extern crate colored;

use std::fmt::Display;
use colored::*;


pub struct Logger {}


impl Logger {
    /// Initializes a new logger
    /// 
    /// # Example
    /// 
    /// ```
    /// use paris::Logger;
    /// let logger = Logger::new();
    /// ```
    pub fn new() -> Logger {
        Logger {}
    }



    /// Prints to stdout and adds some info flair to the text
    /// 
    /// # Example
    /// 
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new();
    /// logger.info("This is some info");
    /// ```
    pub fn info<T: Display>(&self, message: T) -> &Logger {
        println!("{} {}", "[+]".cyan(), message);
        self
    }



    /// Prints to stdout and adds some warning flare to text
    /// 
    /// # Example
    /// 
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new();
    /// logger.warning("This is a warning");
    /// ```
    pub fn warning<T: Display>(&self, message: T) -> &Logger {
        println!("{} {}", "[!]".yellow(), message);
        self
    }

    

    /// Prints to stderr and adds some error flare to text
    /// 
    /// # Example
    /// 
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new();
    /// logger.error("Something broke, here's the error");
    /// ```
    pub fn error<T: Display>(&self, message: T) -> &Logger {
        eprintln!("{} {}", "[-]".red(), message);
        self
    }



    /// Prints a specified amount of newlines to stdout
    /// 
    /// # Example
    /// 
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new();
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
    /// 
    /// ```
    /// # use paris::Logger;
    /// # let logger = Logger::new();
    /// logger
    ///     .indent(1)
    ///     .warning("Indented warning eh? Stands out a bit")
    ///     .newline(5);
    /// ```
    pub fn indent(&self, amount: usize) -> &Logger {
        print!("{}", "\t".repeat(amount));
        self
    }

    

    /// Prints a message to stderr and assumes panic, pass in 
    /// a closure and cleanup if needed, otherwise just kill the program somehow
    /// *TODO: This isn't so well designed tbh, work in progress*
    /// 
    /// # Example
    /// 
    /// ```should_panic
    /// # use paris::Logger;
    /// # let logger = Logger::new();
    /// logger.panic("Everything exploded but I can still pack my bags", |code| {
    ///     panic!("Ending it all now, code: {}", code);
    /// });
    /// ```
    pub fn panic<T, F>(&self, message: T, handler: F)
        where 
            T: Display,
            F: FnOnce(i32)
    {
        let message = message.to_string();
        eprintln!("{} {}\n\n", "[!]".red(), message.bold());

        // Graceful shutdown in this closure, hopefully
        // as graceful as possible atleast.
        // Everything broke code
        handler(0x0100);
    }
}