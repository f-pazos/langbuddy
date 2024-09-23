use std::io;
use std::io::Write;
use std::fmt;

use anyhow::anyhow;

use crate::Preserver;
use crate::browser_session::{self, WordReferenceSpEnSession};

pub struct LanguageBuddy{
    preserver: Preserver,
    session: browser_session::WordReferenceSpEnSession,
    last_word: String,
}

const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";
const WORD_REFERENCE_ENGLISH_GERMAN: &str = "https://www.wordreference.com/ende/";


impl LanguageBuddy {
    // Returns a new LanguageBuddy instance. 
    // 
    // A LanguageBuddy consists of a browser session as well as a Preserver object.
    // The browser session allows the LanguageBuddy to traverse the WordReference
    // website. The Preserver object stores necessary state information about the
    // LanguageBuddy object. This includes saved words and (TODO)historical performance.
    // A Preserver allows the LanguageBuddy to maintain the st  
    pub fn new(output_file: &str) -> anyhow::Result<LanguageBuddy>{
        let preserver = Preserver::read_from_file(output_file)?;

        let session = WordReferenceSpEnSession::new(WORD_REFERENCE_SP_EN_QUERY)?;

        session.lookup("pasar")?;
        if session.get_definition().is_err(){
            println!("couldn't navigate to definition!");
        }

        Ok(
            LanguageBuddy{
                preserver: preserver,
                session: session,
                last_word: String::new(),
            }
        )
    }

    pub fn do_repl(&mut self) {
        loop {
            self.repl();
        }

    }
    pub fn repl(&mut self) -> anyhow::Result<()> { 
            let input = self.parse_input()?;
            match input {
                UserInput::Word(w) => {
                    return self.handle_word(&w);
                },
                UserInput::Command(c) => {
                    return self.handle_command(&c);
                }
            }

            // if matches!(input, UserInput::Word(w)) {
            //     return self.handle_word(w)
            // }
            // if input == UserInput::Word {
            //     self.handle_word()
            // } 

            // match input {
            //     UserInput::Command(c) => {
            //         println!("Command({})", c)
            //     },
            //     UserInput::Word(w) => {
            //         println!("Word({})", w)
            //     }
            // }

            // if word == "\n" {
            //     if last_word.is_empty() || last_word == "\n" {
            //         println!("no previous word to save");
            //         continue;
            //     }
            //     let trimmed = last_word.trim();
            //     self.save(trimmed);

            //     last_word.clear();
            //     continue;
            // }
            // self.lookup_word(&word);
            // last_word = word;

            Ok(())
    }

    fn handle_word(&mut self, word: &str) -> anyhow::Result<()>{
        Ok(())
    }
    fn handle_command(&mut self, command: &Command) -> anyhow::Result<()>{
        Ok(())
    }

    fn save(&mut self, word: &str){
        self.preserver.add_string(word);
        match self.preserver.write() {
            Err(e) => println!("Error writing {} to preserver output: {}", word, e),
            Ok(_e) => println!("{} added to preserver output", word),
        }
    }

    fn lookup_word(&self, word: &str) {
        if self.session.lookup(&word).is_err() {
            println!("Error looking up definition!");
        };

        let val = self.session.get_definition();
        if val.is_err(){
            println!("Error getting definition: {}", val.unwrap_err());
        };
    }

    // parse_input parses the user's input for later use.
    fn parse_input(&self) -> anyhow::Result<UserInput> {
        io::stdout().flush()?;
        let word = input();

        if word.is_err() {
            println!("Problem with input.");
            return Err(anyhow!("oopsies"));
        };

        let word = word.unwrap();

        if word == "save".to_string() {
            return Ok(UserInput::Command(Command::Save));
        }

        return Ok(UserInput::Word(word.to_string()));
    } 
}

enum Command {
    Save,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Save => write!(f, "Save")
        }
    }
}



enum UserInput {
    Command(Command),
    Word(String),
}


fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}
