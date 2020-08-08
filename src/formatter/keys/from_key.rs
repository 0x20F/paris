pub trait FromKey {
    /// Define your own implementation of how
    /// a given key will return the type that
    /// implements it. A wrapper around FromStr
    /// in case the key returns different things based
    /// on some internal value.
    fn from_key(key: &str) -> Option<String>;
}
