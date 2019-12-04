extern crate colored;


pub mod logger {
    use std::fmt::Display;
    use colored::*;

    // TODO: Make these chainable??
    
    pub fn info<T: Display>(message: T) {
        println!("{} {}", "[+]".cyan(), message);
    }

    pub fn attention<T: Display>(message: T) {
        println!("{} {}", "[/]".magenta(), message);
    }

    pub fn warning<T: Display>(message: T) {
        println!("{} {}", "[!]".yellow(), message);
    }

    pub fn error<T: Display>(message: T) {
        eprintln!("{} {}", "[-]".red(), message);
    }

    pub fn panic<T, F>(message: T, handler: F) 
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




#[cfg(test)]
mod tests {
    use super::logger;

    #[test]
    fn it_works() {
        logger::info("This is some basic info");
        logger::info(512512);
        logger::info(String::from("This is from a string"));

        logger::attention("This begs for your attention");
        logger::attention(1337);
        logger::attention(String::from("This begs for your attention from a string"));

        logger::warning("Get warned");
        logger::warning(5125);
        logger::warning(String::from("Get warned from a string"));

        logger::error("This is a basic error");
        logger::error(623462312);
        logger::error(String::from("This is an error from a string"));

        logger::panic("This is all going to explode", |code| assert_eq!(code, 0x0100));
        logger::panic(613123412, |code| assert_eq!(code, 0x0100));
        logger::panic(String::from("This is a panic from a string"), |code| assert_eq!(code, 0x0100));

        // Uncomment to see the output
        // assert!(false);
    }
}