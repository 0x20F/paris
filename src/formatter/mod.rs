mod color;
mod style;
mod ansi;

use colored::Color;
use ansi::ToAnsi;
use regex::Regex;



pub struct Formatter {}


impl Formatter {

    pub fn colorize_string<S>(input: S) -> String
        where S: Into<String>
    {
        lazy_static!(
            static ref TAG: Regex =
                Regex::new(r"<((?:[a-zA-Z-_ ]*+)|/(?:[a-zA-Z-_ ]*+))>")
                .unwrap();
        );

        let input = input.into();

        // Nothing to escape was found
        if TAG.find(&input).is_none() {
            return input;
        }

        let mut output = input.clone();

        for mat in TAG.captures_iter(&input) {
            let key = &mat[0];
            let color = Formatter::cleanup_key(&mat[1]);

            output = output.replace(key, &Color::from_key(&color));
        }

        output
    }


    fn cleanup_key(key: &str) -> String {
        if key.contains(' ') {
            return key.to_string();
        }

        let res: String = key.chars()
            .map(|c| match c {
                '_' => ' ',
                '-' => ' ',
                _ => c
            }).collect();

        res
    }
}







#[cfg(test)]
mod tests {
    use super::*;


    macro_rules! replacement {
        ($name:ident, $key:expr, $code:expr) => {
            #[test]
            fn $name() {
                let k = format!("<{}>", $key);
                let c = format!("\x1B[{}m", $code);

                let s = format!("has this color -> {}", k);
                let parsed = Formatter::colorize_string(s);

                assert!(!parsed.contains(&k));
                assert!(parsed.contains(&c));
            }
        };
    }

    // Foreground checks
    replacement!(cyan, "cyan", 36);
    replacement!(red, "red", 31);
    replacement!(blue, "blue", 34);

    // Background checks
    replacement!(bright, "bright green", 92);
    replacement!(background, "on red", 41);
    replacement!(bright_background, "on bright magenta", 105);

    // Style checks

    // Reset check
    replacement!(reset, "/", 0);


    #[test]
    fn cleanup_key() {
        let color = "on_bright-green";

        let clean = Formatter::cleanup_key(color);

        assert_eq!("on bright green", clean);
    }
}