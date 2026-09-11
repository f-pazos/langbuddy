use std::io::{self, Write};

/**
 * A TerminalClient coordinates program behavior with the terminal.
 */
pub struct TerminalClient;
impl TerminalClient {
    pub fn poll_input(&self) -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).ok();
        return s;
    }

    pub fn display_content(&self, content: Content) {
        print!("{}", content.text);
        io::stdout().flush();
    }
}

pub struct Inputs {
    pub content: String,
}

pub struct Content {
    pub text: String,
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self { text: value }
    }
}
