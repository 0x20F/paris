mod color;

use colored::Color;
use color::ToAnsi;
use regex::Regex;



pub struct Formatter {}


impl Formatter {

    pub fn color_string<S>(string: S) -> String
        where S: Into<String>
    {
        Formatter::replace_keys(string.into())
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
            let color = Formatter::cleanup_color(&mat[1]);

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

        res
    }
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = "<cyan>This <bright-green>is <yellow>a <magenta>string<red> yooo</> with <blue>icons</>";

        let parsed = Formatter::color_string(s);

        println!("{}", parsed);

        assert!(!parsed.contains("<cyan>"));
        assert!(!parsed.contains("<yellow>"));
        assert!(!parsed.contains("<red>"));
        assert!(!parsed.contains("<blue>"));
        assert!(!parsed.contains("<bright-green>"));
        assert!(!parsed.contains("</>"));
    }
}