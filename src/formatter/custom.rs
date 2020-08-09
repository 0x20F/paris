use crate::formatter::keys::{FromKey, Key};

pub struct CustomStyle {
    key: String,
    colors: Vec<String>
}

impl CustomStyle {
    pub fn new(key: &str, colors: Vec<&str>) -> Self {
        Self {
            key: format!("<{}>", key),
            colors: colors.iter().map(|s| s.to_string()).collect()
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn expand(&self) -> String {
        let mut colors: Vec<String> = Vec::with_capacity(2);

        // Turn it into the ansi values it should be
        for color in self.colors.iter() {
            let key = Key::new(color);
            let ansi = key.to_ansi();

            colors.push(ansi);
        }

        colors.join("")
    }
}