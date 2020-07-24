mod key_list;

pub use key_list::KeyList;

/// Escape whatever's being sent
/// in here to an ansi code
pub struct Ansi {}

impl Ansi {
    /// Add the required escape and terminator characters to
    /// an ansi code.
    pub fn escape(code: u8) -> String {
        format!("\x1B[{}m", code)
    }
}

pub trait FromKey {
    /// Define your own implementation of how
    /// a given key will return the type that
    /// implements it. A wrapper around FromStr
    /// in case the key returns different things based
    /// on some internal value.
    fn from_key(key: &str) -> Option<String>;
}
