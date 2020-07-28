pub struct CustomStyle {
    key: String,
    colors: Vec<String>
}

impl CustomStyle {
    pub fn new(key: &str, colors: Vec<String>) -> Self {
        Self {
            key: key.to_owned(),
            colors
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn expand(&self) -> String {
        // Turn it into the ansi values it should be
        unimplemented!()
    }
}