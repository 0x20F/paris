/// Extends the functionality of color or style enums
/// so they can convert directly to ansi escaped codes
/// and properly parse based on my custom keys
pub trait ToAnsi {
    fn from_key(key: &str) -> String;

    /// Add the required escape and terminator characters to
    /// an ansi code.
    fn escape(code: &str) -> String {
        let mut res = String::from("\x1B[");

        res.push_str(code);

        res.push('m');
        res
    }
}