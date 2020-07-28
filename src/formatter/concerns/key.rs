use super::FromKey;
use crate::formatter::color::Color;
use crate::formatter::style::Style;
use crate::formatter::icons::LogIcon;
use crate::formatter::custom::CustomStyle;
use std::fmt::{Display, Formatter, Result};


pub struct Key<'a> {
    contents: &'a str,
    clean: String
}

impl<'a> Key<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            contents: value,
            clean: Self::clean(value)
        }
    }

    pub fn contents(&self) -> &str {
        self.contents
    }

    pub fn as_color(&self) -> Option<String> {
        Color::from_key(&self.clean)
    }

    pub fn as_style(&self) -> Option<String> {
        Style::from_key(&self.clean)
    }

    pub fn as_icon(&self) -> Option<String> {
        LogIcon::from_key(&self.clean)
    }

    pub fn to_ansi(&self) -> String {
        let mut content: String = self.contents().to_owned();

        if let Some(c) = self.as_color() {
            content = c;
        }

        if let Some(s) = self.as_style() {
            content = s;
        }

        if let Some(i) = self.as_icon() {
            content = i;
        }

        content
    }

    /// Removes characters that can be used instead
    /// of spaces from a key if the key doesn't already
    /// contain spaces
    fn clean(key: &str) -> String {
        let key = key.trim_matches(|c| c == '<' || c == '>');

        // If key already contains space, its already
        // intended or a typo
        if key.contains(' ') {
            return key.to_string();
        }

        key.chars()
            .map(|c| match c {
                '_' => ' ',
                '-' => ' ',
                _ => c,
            })
            .collect()
    }
}

impl Display for Key<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup() {
        let color = "<on_bright-green>";

        let clean = Key::clean(color);

        assert_eq!("on bright green", clean);
    }
}
