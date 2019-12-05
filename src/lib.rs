extern crate colored;

use std::fmt::Display;
use colored::*;




pub struct Logger {}


impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }


    pub fn info<T: Display>(&self, message: T) -> &Logger {
        println!("{} {}", "[+]".cyan(), message);
        self
    }

    
    pub fn attention<T: Display>(&self, message: T) -> &Logger {
        println!("{} {}", "[/]".magenta(), message);
        self
    }

    
    pub fn warning<T: Display>(&self, message: T) -> &Logger {
        println!("{} {}", "[!]".yellow(), message);
        self
    }

    
    pub fn error<T: Display>(&self, message: T) -> &Logger {
        eprintln!("{} {}", "[-]".red(), message);
        self
    }

    
    pub fn panic<T, F>(&self, message: T, handler: F)
        where 
            T: Display,
            F: FnOnce(i16)
    {
        let message = message.to_string();
        eprintln!("{} {}\n\n", "[!]".red(), message.bold());

        // Everything broke code
        handler(0x0100);
    }
}