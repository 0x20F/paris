extern crate colored;

use std::fmt::{ Display, Formatter, Result };
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
    
    /// ❤️
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