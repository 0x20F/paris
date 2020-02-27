use colored::Color;
use regex::Regex;



struct Colors {}


impl Colors {
    pub fn get(key: &str) -> String {
        let is_bg = key.starts_with("on");
        let is_reset = key == "/";

        if is_reset {
            return Colors::escape("0");
        }

        let key = key.trim_start_matches("on ");
        let color: Color = Colors::get_color(key);

        if is_bg {
            return Colors::escape(color.to_bg_str());
        }

        Colors::escape(color.to_fg_str())
    }



    fn get_color(key: &str) -> Color {
        let color: Color = key.into();

        color
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

            output = output.replace(key, &Colors::get(color));
        }

        output
    }
}