use super::concerns::{Ansi, FromKey};
use std::str::FromStr;

pub enum Style {
    Bold,
    BoldReset,

    Italic,
    ItalicReset,

    Underline,
    UnderlineReset,

    Dimmed,
    DimmedReset,

    None,
}

impl Style {
    pub fn get_value(&self) -> u8 {
        match self {
            Style::Bold => 1,
            Style::BoldReset => 22,

            Style::Dimmed => 2,
            Style::DimmedReset => 22,

            Style::Italic => 3,
            Style::ItalicReset => 23,

            Style::Underline => 4,
            Style::UnderlineReset => 24,

            Style::None => 0,
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
            "bold" | "b" => Ok(Style::Bold),
            "/bold" | "/b" => Ok(Style::BoldReset),

            "dimmed" | "d" => Ok(Style::Dimmed),
            "/dimmed" | "/d" => Ok(Style::DimmedReset),

            "italic" | "i" => Ok(Style::Italic),
            "/italic" | "/i" => Ok(Style::ItalicReset),

            "underline" | "u" => Ok(Style::Underline),
            "/underline" | "/u" => Ok(Style::UnderlineReset),

            _ => Err(()),
        }
    }
}

impl FromKey for Style {
    fn from_key(key: &str) -> Option<String> {
        let s = Style::from(key);

        match s {
            Style::None => None,
            _ => Some(Ansi::escape(s.get_value())),
        }
    }
}
