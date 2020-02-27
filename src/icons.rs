//! Contains icons that can be used when
//! outputting to the terminal. All icons are printable
//! and can be converted to strings.
//!
//! There are no tags that can be used
//! in log strings on the other hand. So you can't
//! write `<info>` in a string and expect it to
//! be replaced with the info icon.
//! Not at the moment at least.
use std::fmt::{ Display, Formatter, Result };




/// Contains definitions for icons that can be
/// used in the terminal. See [this github repo](https://github.com/sindresorhus/figures)
/// for an entire list. Use this in combination with printing macros.
pub enum LogIcon {
    /// A check mark, use when things go well
    ///
    /// # Example
    /// ```
    /// # use paris::icons::LogIcon;
    /// println!("{} Everything went well", LogIcon::Tick);
    /// // ✔ Everything went well
    /// ```
    Tick,

    /// A cross, use when things go bad, or be creative
    ///
    /// # Example
    /// ```
    /// # use paris::icons::LogIcon;
    /// println!("{} Oops, try again!", LogIcon::Cross);
    /// // ✖ Oops, try again!
    /// ```
    Cross,

    /// A fancy 'i', for information
    ///
    /// # Example
    /// ```
    /// # use paris::icons::LogIcon;
    /// println!("{} In Switzerland it is illegal to own just one guinea pig", LogIcon::Info);
    /// // ℹ In Switzerland it is illegal to own just one guinea pig.
    /// ```
    Info,

    /// A triangle with an exclamation mark in it, dangerous
    ///
    /// # Example
    /// ```
    /// # use paris::icons::LogIcon;
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