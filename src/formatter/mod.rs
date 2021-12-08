//! A wrapper around a few functions to make
//! finding and replacing keys inside a string easier.

mod color;
mod concerns;
mod icons;
mod keys;
mod style;

#[cfg(not(feature = "no_logger"))]
mod custom;
#[cfg(not(feature = "no_logger"))]
use custom::CustomStyle;
#[cfg(not(feature = "no_logger"))]
use keys::Key;

use keys::KeyList;

pub use concerns::Ansi;
pub use icons::LogIcon;

/// Heavier formatter that allows the possibility of
/// custom styles in strings. That is the only reason
/// this struct exists, if you don't need custom things
/// just use the `colorize_string()` function provided
/// in the module.
#[cfg(not(feature = "no_logger"))]
pub struct Formatter<'a> {
    custom_styles: Vec<CustomStyle<'a>>,
}

#[cfg(not(feature = "no_logger"))]
impl<'a> Default for Formatter<'a> {
    fn default() -> Self {
        Self {
            custom_styles: vec![],
        }
    }
}

#[cfg(not(feature = "no_logger"))]
impl<'a> Formatter<'a> {
    /// Create a new formatter with no custom styles defined
    pub fn new() -> Self {
        Self::default()
    }

    /// Tell the formatter that you want a new style
    /// and what colors that style equates to so it knows
    /// what to replace it with when formatting
    ///
    /// # Example
    /// ```
    /// use paris::formatter::Formatter;
    ///
    /// let mut fmt = Formatter::new();
    /// fmt.new_style("lol", vec!["green", "bold", "on_blue"]);
    ///
    /// // '<lol>' is now a key that can be used in strings
    pub fn new_style(&mut self, key: &str, colors: Vec<&'a str>) -> &mut Self {
        self.custom_styles.push(CustomStyle::new(key, colors));

        self
    }

    /// Finds all keys in the given input. Keys meaning
    /// whatever the logger uses. Something that looks like `<key>`.
    /// And replaces all those keys with their color, style
    /// or icon equivalent.
    pub fn colorize(&self, input: &str) -> String {
        let mut output = input.to_string();

        for key in KeyList::new(&input) {
            if let Some(style) = self.as_style(&key) {
                let ansi = style.expand();
                output = output.replace(&key.to_string(), &ansi);
            }

            output = output.replace(&key.to_string(), &key.to_ansi());
        }

        output
    }

    /// Convert a key to a custom style if they match
    fn as_style(&self, key: &Key) -> Option<&CustomStyle> {
        for style in self.custom_styles.iter() {
            if style.key() == key.contents() {
                return Some(style);
            }
        }

        None
    }
}

/// Finds all keys in the given input. Keys meaning
/// whatever the logger uses. Something that looks like `<key>`.
/// And replaces all those keys with their color, style
/// or icon equivalent.
///
/// #### This function does not take into account custom styles, you need the struct for that
pub fn colorize_string<S>(input: S) -> String
where
    S: Into<String>,
{
    let input = input.into();
    let mut output = input.clone();

    for key in KeyList::new(&input) {
        output = output.replace(&key.to_string(), &key.to_ansi());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! replacement {
        ($key:ident, $code:expr) => {
            #[test]
            fn $key() {
                let n = stringify!($key);

                let k = format!("<{}>", n);
                let c = format!("\x1B[{}m", $code);

                let s = format!("has: {:<20} -> {}Test string", n, k);
                let parsed = colorize_string(s);

                // Just to see all the cool colors
                println!("{}", parsed);

                assert!(!parsed.contains(&k));
                assert!(parsed.contains(&c));
            }
        };
    }

    // Color checks
    replacement!(black, 30);
    replacement!(red, 31);
    replacement!(green, 32);
    replacement!(yellow, 33);
    replacement!(blue, 34);
    replacement!(magenta, 35);
    replacement!(cyan, 36);
    replacement!(white, 37);

    // Bright color checks
    replacement!(bright_black, 90);
    replacement!(bright_red, 91);
    replacement!(bright_green, 92);
    replacement!(bright_yellow, 93);
    replacement!(bright_blue, 94);
    replacement!(bright_magenta, 95);
    replacement!(bright_cyan, 96);
    replacement!(bright_white, 97);

    // Background normal
    replacement!(on_black, 40);
    replacement!(on_red, 41);
    replacement!(on_green, 42);
    replacement!(on_yellow, 43);

    // Background bright
    replacement!(on_bright_black, 100);
    replacement!(on_bright_red, 101);
    replacement!(on_bright_green, 102);
    replacement!(on_bright_yellow, 103);

    // Style checks
    replacement!(bold, 1);
    replacement!(dimmed, 2);
    replacement!(italic, 3);
    replacement!(underline, 4);
    replacement!(blink, 5);
    replacement!(reverse, 7);
    replacement!(hidden, 8);
    replacement!(strikethrough, 9);

    // Reset check
    #[test]
    fn reset() {
        let k = "</>";
        let c = format!("\x1B[{}m", 0);

        let s = format!("{}Test string", k);
        let parsed = colorize_string(s);

        assert!(!parsed.contains(&k));
        assert!(parsed.contains(&c));
    }

    #[test]
    fn normal_tags() {
        let s = String::from("<html> This is normal stuff </html>");
        let parsed = colorize_string(s);

        // Make sure its still in there
        assert!(parsed.contains("<html>"));
    }

    #[test]
    #[cfg(not(feature = "no_logger"))]
    fn custom_style() {
        let s =
            String::from("<custom> This has custom styles <lol> Here's some blue shit yoooo </>");

        let mut fmt = Formatter::new();
        fmt.new_style("custom", vec!["red", "on-green"])
            .new_style("lol", vec!["cyan", "on-blue"]);

        let parsed = fmt.colorize(&s);

        assert!(!parsed.contains("<custom>"));
        assert!(!parsed.contains("<lol>"));
    }
}
