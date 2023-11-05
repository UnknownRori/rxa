/// A very simple to create a clickable link in terminal
/// currently doesn't check if terminal support it or not
use std::fmt;

pub struct Link<'a> {
    text: &'a str,
    url: &'a str,
}

impl<'a> Link<'a> {
    pub fn new(text: &'a str, url: &'a str) -> Self {
        Self { text, url }
    }
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
            self.url, self.text
        )
    }
}
