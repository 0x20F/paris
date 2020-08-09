//! Simple way to output beautiful text in your
//! CLI applications. Only limit is your imagination.
//!
//! # How to use
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     use paris::Logger;
//!
//!     let mut log = Logger::new();
//!
//!     log.info("It's that simple!");
//!     # }
//!
//! ### Simple api
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     // You can have icons at the start of your message!
//!     log.info("Will add ℹ at the start");
//!     log.error("Will add ✖ at the start");
//!     # }
//!
//!
//!     # #[cfg(feature = "macros")] {
//!     # use paris::{ info, error };
//!
//!      // or as macros
//!     info!("Will add ℹ at the start");
//!     error!("Will add ✖ at the start");
//!     # }
//!
//!
//! See [the Logger struct](https://docs.rs/paris/) for all methods
//!
//!
//! # Chaining
//! All methods can be chained together to build more intricate
//! log/message combinations, in hopes of minimizing the chaos
//! that every log string becomes when you have to concatenate
//! a bunch of strings and add tabs and newlines everywhere.
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("this is some info")
//!        .indent(4).warn("this is now indented by 4")
//!        .newline(5)
//!        .success("and this is 5 lines under all other messages");
//!     # }
//!
//! # Customisation
//! Outputting text is cool. Outputting text with a colored icon
//! at the start is even cooler! But this crate is all about
//! customisation, about making the logs feel like home, if you will.
//! Included in the crate are a variety of keys you can use
//! to colorize your logs just the way you want them to be.
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("I can write normal text or use tags to <red>color it</>");
//!     log.warn("Every function can contain <on-green><black>tags</>");
//!
//!     log.info("If you don't write them <what>correctly</>, you just get an ugly looking tag");
//!     # }
//!
//! There's a key for all colors supported by the terminal `(white, black, red, blue, magenta, etc.)`
//! If you add the word `on` to any of those colors, it becomes the
//! background color instead `(on red, on blue, on green)`.
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     // How useful...
//!     log.info("<on-red> This has red background </>");
//!     # }
//!
//! Maybe you'd like to use your terminals brighter colors, if that's the case
//! you just have to add `bright` to your tag. Makes sense.
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.info("<blue><on-bright-red> This text is blue on a bright red background</> it's a pain");
//!     # }
//!
//! If you feel like writing a lot of colors by hand is too tedious, or if you know you're going
//! to be using the same combination of colors over and over again you can create a `custom style`
//! that encapsulates all those colors.
//!
//!     # #[cfg(not(feature = "no_logger"))] {
//!     # use paris::Logger;
//!     # let mut log = Logger::new();
//!     log.add_style("lol", vec!["green", "bold", "on-bright-blue"]);
//!
//!     // '<lol>' is now a key that you can use in your strings
//!     log.info("<lol>This is has all your new styles</>");
//!     # }
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
//!
//! ### Macros
//! With the macros feature enabled, you get access to macro equivalents
//! of the logger functions.
//!
//! Advantages of using macros:
//! * You don't have to instantiate the logger `Logger::new()`
//! * Simple to write
//! * Can format parameters like `print!` and `println!`
//!
//! Disadvantages of using macros:
//! * Can't chain calls
//! * Manual newlines and tabs with `\n` and `\t`
//! * There's no loading animation for macros
//!
//! You get to decide whether you want to use macros or not.
//! Every macro has the same functionality as its `Logger`
//! equivalent. Colors and icon keys work just the same.
//!
//! See [the Logger struct](https://docs.rs/paris/) for all methods and their macro equivalents
#![warn(missing_docs)]

#[cfg(feature = "timestamps")]
mod timestamp;

#[cfg(feature = "macros")]
mod macros;

#[cfg(not(feature = "no_logger"))]
mod logger;
#[cfg(not(feature = "no_logger"))]
pub use logger::Logger;

pub mod formatter;
pub mod output;

pub use formatter::LogIcon;
