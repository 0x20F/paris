use crate::formatter::keys::Key;

pub struct CustomStyle<'a> {
    key: String,
    colors: Vec<String>,
    test: &'a str
}

impl<'a> CustomStyle<'a> {
    pub fn new(key: &str, colors: Vec<&str>) -> Self {
        Self {
            key: format!("<{}>", key),
            colors: colors.iter().map(|s| (*s).to_string()).collect(),
            test: "as"
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

#[cfg(test)]
mod tests {
    use crate::formatter::keys::Key;
    use crate::formatter::custom::CustomStyle;

    #[test]
    fn ansi_expansion() {
        let style = CustomStyle::new("lol", vec!["blue"]);
        let color = Key::new("blue");

        assert_eq!(style.expand(), color.to_ansi());
    }
}