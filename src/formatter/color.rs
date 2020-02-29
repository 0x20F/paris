use colored::*;



/// Extends the functionality of colored::Color
/// so it can convert directly to ansi escaped codes
/// and properly parse based on my custom keys
pub trait ToAnsi {
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