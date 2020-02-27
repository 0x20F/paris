use colored::Color;



pub struct Colors {}


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