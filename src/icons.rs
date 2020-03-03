//! Contains icons that can be used when
//! outputting to the terminal. All icons are printable
//! and can be converted to strings.
//!
//! There are no tags that can be used
//! in log strings on the other hand. So you can't
//! write `<info>` in a string and expect it to
//! be replaced with the info icon.
use std::str::FromStr;


/// Contains definitions for icons that can be
/// used in the terminal. See [this github repo](https://github.com/sindresorhus/figures)
/// for an entire list. Use this in combination with printing macros.
pub enum LogIcon {
    /// A check mark, use when things go well
    ///
    /// # Example
    /// ```
    /// use paris::LogIcon;
    ///
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
    Heart,



    None // So there's a fallback
}



impl LogIcon {
    /// Match the enum value and return the equivalent icon.
    /// On Windows, icons will be replaced with other *things* that
    /// are supported. See [this github repo](https://github.com/sindresorhus/figures)
    /// for all replacements
    pub fn to_str<'a>(&self) -> &'a str {

        if cfg!(windows) {
            return match self {
                LogIcon::Info => "i",
                LogIcon::Cross => "×",
                LogIcon::Warning => "‼",
                LogIcon::Tick => "√",
                LogIcon::Heart => "♥",
                LogIcon::None => ""
            }
        }

        match self {
            LogIcon::Info => "ℹ",
            LogIcon::Cross => "✖",
            LogIcon::Warning => "⚠",
            LogIcon::Tick => "✔",
            LogIcon::Heart => "♥",
            LogIcon::None => ""
        }
    }
}



impl<'a> From<&'a str> for LogIcon {
    fn from(s: &'a str) -> Self {
        s.parse().unwrap_or(LogIcon::None)
    }
}



impl FromStr for LogIcon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_ref() {
            "info" => Ok(LogIcon::Info),
            "cross" => Ok(LogIcon::Cross),
            "warn" => Ok(LogIcon::Warning),
            "tick" => Ok(LogIcon::Tick),
            "heart" => Ok(LogIcon::Heart),
            _ => Err(())
        }
    }
}