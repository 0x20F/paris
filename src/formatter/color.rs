use super::ansi::ToAnsi;
use colored::*;



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
            return Color::escape(color.to_bg_str());
        }

        Color::escape(color.to_fg_str())
    }
}






#[cfg(test)]
mod tests {
    use super::*;


    macro_rules! color_test {
        ($name:ident, $key:expr, $value:expr) => {
            #[test]
            fn $name() {
                let v = String::from(format!("\x1B[{}m", $value));
                let c = Color::from_key($key);

                assert_eq!(c, v);
            }
        };
    }


    color_test!(reset, "/", 0);
    color_test!(background, "on red", 41);
    color_test!(foreground, "red", 31);
}