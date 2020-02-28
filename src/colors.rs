use colored::Color;
use regex::Regex;


/// Extends the functionality of colored::Color
/// so it can convert directly to ansi escaped codes
/// and properly parse based on my custom keys
trait ToAnsi {
    fn get(key: &str) -> String;
    fn escape(code: &str) -> String;
}


impl ToAnsi for Color {

    ///
    /// Convert a str to an Ansi color code based
    /// on the key standard that the colored lib has.
    /// With some of my own additions.
    ///
    /// # Example
    /// If "red" is passed, it'll become the red foreground
    /// color code. If "on red" is passed, it'll become the
    /// red background color code.
    fn get(key: &str) -> String {
        let is_bg = key.starts_with("on");
        let is_reset = key == "/";

        if is_reset {
            return Color::escape("0");
        }

        let color: Color = key.trim_start_matches("on ").into();

        if is_bg {
            return Color::escape(color.to_bg_str());
        }

        Color::escape(color.to_fg_str())
    }



    /// Add the required escape and terminator characters to
    /// an ansi color code.
    fn escape(code: &str) -> String {
        let mut res = String::from("\x1B[");

        res.push_str(code);

        res.push('m');
        res
    }
}



pub struct Parser {}


impl Parser {

    pub fn parse_color_string<S>(string: S) -> String
        where S: Into<String>
    {
        Parser::replace_keys(string.into())
    }


    fn replace_keys(input: String) -> String {
        lazy_static!(
            static ref TAG: Regex =
                Regex::new(r"<((?:[a-zA-Z-_ ]*+)|/(?:[a-zA-Z-_ ]*+))>")
                .unwrap();
        );

        // Nothing to escape was found
        if TAG.find(&input).is_none() {
            return input;
        }

        let mut output = input.clone();

        for mat in TAG.captures_iter(&input) {
            let key = &mat[0];
            let color = Parser::cleanup_color(&mat[1]);

            output = output.replace(key, &Color::get(&color));
        }

        output
    }


    fn cleanup_color(key: &str) -> String {
        if key.contains(' ') {
            return key.to_string();
        }

        let res: String = key.chars()
            .map(|c| match c {
                '_' => ' ',
                '-' => ' ',
                _ => c
            }).collect();

        println!("Res is: {}", res);

        res
    }
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = "<cyan>This <bright-green>is <yellow>a <magenta>string<red> yooo</> with <blue>icons</>";

        let parsed = Parser::parse_color_string(s);

        println!("{}", parsed);

        assert!(!parsed.contains("<cyan>"));
        assert!(!parsed.contains("<yellow>"));
        assert!(!parsed.contains("<red>"));
        assert!(!parsed.contains("<blue>"));
        assert!(!parsed.contains("<bright-green>"));
        assert!(!parsed.contains("</>"));
    }
}