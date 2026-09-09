use std::io::{self, Write};

/**
 * A TerminalInterface coordinates program behavior with the terminal.
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

pub struct Content {
    pub text: String,
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self { text: value }
    }
}

// fn field_input(&self) -> anyhow::Result<()> {
//     Ok(())
//     io::stdout().flush()?;
//     let word = input();

//     // if word.is_err() {
//     //     return Err(anyhow!("problem receiving input: {}", word.unwrap_err()));
//     // };

//     // let word = word.unwrap();
//     // let word = word.trim();

//     // if word == "save" {
//     //     return Ok(UserInput::Command(Command::Save));
//     // }

//     // return Ok(UserInput::Word(word.to_string()));
// }

// fn input() -> anyhow::Result<String> {
//     let mut s = String::new();
//     io::stdin().read_line(&mut s)?;
//     return Ok(s);
// }
