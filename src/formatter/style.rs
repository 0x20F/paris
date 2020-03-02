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
    pub fn get_value(&self) -> u8 {
        match self {
            Style::Bold => 1,
            Style::Dimmed => 2,
            Style::Italic => 3,
            Style::Underline => 4,
            Style::None => 0
        }
    }
}



impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
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



impl ToAnsi for Style {
    fn from_key(key: &str) -> Option<String> {
        let s = Style::from(key);

        match s {
            Style::None => None,
            _ => Some(Style::escape(s.get_value()))
        }
    }
}
