pub trait FromKey {
    fn from_key(key: &str) -> Option<String>;
}