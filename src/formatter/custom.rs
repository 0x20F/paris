pub struct CustomStyle {
    key: String,
    value: String
}

impl CustomStyle {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned()
        }
    }
}