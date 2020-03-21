mod color;
mod style;
mod icons;
mod concerns;

use color::Color;
use style::Style;

use concerns::FromKey;
pub use icons::LogIcon;


/// A wrapper around a few functions to make
/// finding and replacing keys inside a string easier.
pub struct Formatter {}


impl Formatter {

    /// Finds all keys in the given input. Keys meaning
    /// whatever the logger uses. Something that looks like `<key>`.
    /// And replaces all those keys with their color, style
    /// or icon equivalent.
    pub fn colorize_string<S>(input: S) -> String
        where S: Into<String>
    {
        let input = input.into();
        let mut output = input.clone();

        for key in Finder::new(&input) {
            let color_key = Formatter::cleanup_key(key);

            let c = Color::from_key(&color_key);
            if let Some(c) = c {
                output = output.replace(key, &c);
                continue;
            }


            let s = Style::from_key(&color_key);
            if let Some(c) = s {
                output = output.replace(key, &c);
                continue;
            }


            let i = LogIcon::from_key(&color_key);
            if let Some(i) = i {
                output = output.replace(key, &i);
                continue;
            }
        }

        output
    }


    /// Removes characters that can be used instead
    /// of spaces from a key if the key doesn't already
    /// contain spaces
    fn cleanup_key(key: &str) -> String {
        let key = key.trim_matches(|c| c == '<' || c == '>');

        // If key already contains space, its already
        // intended or a typo
        if key.contains(' ') {
            return key.to_string();
        }

        key.chars()
            .map(|c| match c {
                '_' => ' ',
                '-' => ' ',
                _ => c
            }).collect()
    }
}




struct Finder<'a> {
    input: &'a str,
}


impl<'a> Finder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}


impl<'a> Iterator for Finder<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;

        if let Some(i) = self.input.find('<') {
            let rest = self.input.char_indices()
                .skip(i)
                .take_while(|(_, c)| *c != '>')
                .last()
                .map(|(idx, c)| idx + c.len_utf8())
                .unwrap_or_default();

            // +1 to get the last '>' that's excluded
            key = Some(&self.input[i..(rest + 1)]);
            self.input = &self.input[(rest + 1)..];
        }

        key
    }
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
                let parsed = Formatter::colorize_string(s);

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

    // Reset check
    #[test]
    fn reset() {
        let k = "</>";
        let c = format!("\x1B[{}m", 0);

        let s = format!("{}Test string", k);
        let parsed = Formatter::colorize_string(s);

        assert!(!parsed.contains(&k));
        assert!(parsed.contains(&c));
    }

    #[test]
    fn normal_tags() {
        let s = String::from("<html> This is normal stuff </html>");
        let parsed = Formatter::colorize_string(s);

        // Make sure its still in there
        assert!(parsed.contains("<html>"));
    }

    #[test]
    fn cleanup_key() {
        let color = "on_bright-green";

        let clean = Formatter::cleanup_key(color);

        assert_eq!("on bright green", clean);
    }
}