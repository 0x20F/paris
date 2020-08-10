/// Escape whatever's being sent
/// in here to an ansi code
pub struct Ansi {}

impl Ansi {
    /// Add the required escape and terminator characters to
    /// an ansi code.
    pub fn escape(code: u8) -> String {
        format!("\x1B[{}m", code)
    }

    /// Clears the line of all characters
    pub fn clear_line() {
        print!("\r\x1B[2K");
    }
}
