use super::ansi::ToAnsi;
use std::str::FromStr;


pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
    None // So we can check if its a color or not
    // In case there's HTML in the logs
}



impl Color {
    pub fn get_fg_value(&self) -> u8 {
        match *self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
            Color::Reset => 0,
            Color::None => 0
        }
    }


    pub fn get_bg_value(&self) -> u8 {
        match *self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::BrightBlack => 100,
            Color::BrightRed => 101,
            Color::BrightGreen => 102,
            Color::BrightYellow => 103,
            Color::BrightBlue => 104,
            Color::BrightMagenta => 105,
            Color::BrightCyan => 106,
            Color::BrightWhite => 107,
            Color::Reset => 0,
            Color::None => 0
        }
    }
}



impl<'a> From<&'a str> for Color {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(Color::None)
    }
}



impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_ref() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "bright black" => Ok(Color::BrightBlack),
            "bright red" => Ok(Color::BrightRed),
            "bright green" => Ok(Color::BrightGreen),
            "bright yellow" => Ok(Color::BrightYellow),
            "bright blue" => Ok(Color::BrightBlue),
            "bright magenta" => Ok(Color::BrightMagenta),
            "bright cyan" => Ok(Color::BrightCyan),
            "bright white" => Ok(Color::BrightWhite),
            "/" => Ok(Color::Reset),
            _ => Err(())
        }
    }
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

        let color = Color::from(key.trim_start_matches("on "));

        if is_bg {
            return Color::escape(color.get_bg_value());
        }

        Color::escape(color.get_fg_value())
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