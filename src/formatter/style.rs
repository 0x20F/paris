use super::ansi::ToAnsi;
use std::str::FromStr;


pub enum Style {
    Bold,
    Italic,
    Underline,
    Dimmed,
    None
}



impl Style {
    pub fn to_str<'a>(self) -> &'a str {
        match self {
            Style::Bold => "1",
            Style::Dimmed => "2",
            Style::Italic => "3",
            Style::Underline => "4",
            Style::None => ""
        }
    }
}



impl ToAnsi for Style {
    fn from_key(key: &str) -> String {
        let s = Style::from(key);

        return Style::escape(s.to_str())
    }
}



impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        s.parse().unwrap_or(Style::None)
    }
}



impl From<String> for Style {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(Style::None)
    }
}



impl FromStr for Style {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let src = s.to_lowercase();

        match src.as_ref() {
            "bold" => Ok(Style::Bold),
            "italic" => Ok(Style::Italic),
            "underline" => Ok(Style::Underline),
            "dimmed" => Ok(Style::Dimmed),
            _ => Err(())
        }
    }
}