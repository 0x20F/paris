use colored::Color;
use regex::Regex;



trait ToAnsi {
    fn get(key: &str) -> String;
    fn escape(code: &str) -> String;
}


impl ToAnsi for Color {
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
        Parser::replace_colors(string.into())
    }


    fn replace_colors(input: String) -> String {
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
            let color = &mat[1];

            output = output.replace(key, &Color::get(color));
        }

        output
    }
}