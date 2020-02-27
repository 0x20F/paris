use colored::Color;


pub struct Colors {}


impl Colors {
    pub fn get(color: &str) -> String {
        if color == "/" {
            return String::from("\x1B[0m");
        }

        if color.starts_with("bg") {
            let color = color.split_whitespace().last();

            Colors::bg(color.unwrap())
        } else {
            Colors::fg(color)
        }
    }


    pub fn fg(color: &str) -> String {
        let color: Color = color.into();

        let mut res = String::from("\x1B[");
        res.push_str(color.to_fg_str());

        res.push('m');
        res
    }


    pub fn bg(color: &str) -> String {
        let color: Color = color.into();

        let mut res = String::from("\x1B[");
        res.push_str(color.to_bg_str());

        res.push('m');
        res
    }
}



// Use for adding bold, underline, background colors, foreground colors
pub struct Formatter {}


impl Formatter {

}