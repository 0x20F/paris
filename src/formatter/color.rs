use colored::*;



/// Extends the functionality of colored::Color
/// so it can convert directly to ansi escaped codes
/// and properly parse based on my custom keys
pub trait ToAnsi {
    fn from_key(key: &str) -> String;

    fn escape(code: &str) -> String;

    fn escape_bg(&self) -> String;
    fn escape_fg(&self) -> String;
}


impl ToAnsi for Color {

    /// Convert a str to an Ansi color code based
    /// on the key standard that the colored lib has.
    /// With some of my own additions.
    ///
    /// # Example
    /// If "red" is passed, it'll become the red foreground
    /// color code. If "on red" is passed, it'll become the
    /// red background color code.
    fn from_key(key: &str) -> String {
        let is_bg = key.starts_with("on");
        let is_reset = key == "/";

        if is_reset {
            return Color::escape("0");
        }

        let color = Color::from(key.trim_start_matches("on "));

        if is_bg {
            return color.escape_bg();
        }

        color.escape_fg()
    }



    /// Add the required escape and terminator characters to
    /// an ansi color code.
    fn escape(code: &str) -> String {
        let mut res = String::from("\x1B[");

        res.push_str(code);

        res.push('m');
        res
    }



    fn escape_bg(&self) -> String {
        Color::escape(self.to_bg_str())
    }



    fn escape_fg(&self) -> String {
        Color::escape(self.to_fg_str())
    }
}






#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_reset() {
        let reset = String::from("\x1B[0m");

        let color = Color::from_key("/");

        assert_eq!(reset, color);
    }


    #[test]
    fn get_bg_color() {
        let red = String::from("\x1B[41m");

        let color = Color::from_key("on red");

        assert_eq!(red, color);
    }


    #[test]
    fn get_fg_color() {
        let red = String::from("\x1B[31m");

        let color = Color::from_key("red");

        assert_eq!(red, color);
    }


    #[test]
    fn escape_bg() {
        let red = String::from("\x1B[41m");

        let color = Color::Red.escape_bg();

        assert_eq!(red, color);
    }


    #[test]
    fn escape_fg() {
        let red = String::from("\x1B[31m");

        let color = Color::Red.escape_fg();

        assert_eq!(red, color);
    }
}