extern crate colored;


pub mod storage {
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
            eprintln!("{} {}", "[!]".red(), message.bold());
    
            // Everything broke code
            handler(0x0100);
        }
    }
}




#[cfg(test)]
mod tests {
    use super::storage::Logger;

    #[test]
    fn it_works() {
        let logger = Logger::new();
        let error_code = 0x0100;

        logger
            .info("This is some basic info")
            .info(512512)
            .info(String::from("This is from a string"));

        logger
            .attention("This begs for your attention")
            .attention(1337)
            .attention(String::from("This begs for your attention from a string"));

        logger
            .warning("Get warned")
            .warning(5125)
            .warning(String::from("Get warned from a string"));

        logger
            .error("This is a basic error")
            .error(623462312)
            .error(String::from("This is an error from a string"));

        logger.panic("This is all going to explode", |code| assert_eq!(code, error_code));
        logger.panic(613123412, |code| assert_eq!(code, error_code));
        logger.panic(String::from("This is a panic from a string"), |code| assert_eq!(code, error_code));

        logger
            .info("all")
            .attention("together")
            .warning("hey")
            .error("ho")
            .panic("Panic last because your program should die when this gets called!", |code| assert_eq!(code, error_code));

        // Uncomment to see the output
        // assert!(false);
    }
}