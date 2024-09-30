use std::io;
use std::io::Write;
use std::fmt;

use anyhow::anyhow;

use crate::Preserver;
use crate::browser_session::{self, WordReferenceSpEnSession};

pub struct LanguageBuddy{
    preserver: Preserver,
    session: browser_session::WordReferenceSpEnSession,
    current_word: String,
}

const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";
const WORD_REFERENCE_ENGLISH_GERMAN: &str = "https://www.wordreference.com/ende/";


impl LanguageBuddy {
    /**
     * Returns a new LanguageBuddy instance. 
     * 
     * A LanguageBuddy consists of a browser session as well as a Preserver object.
     * The browser session allows the LanguageBuddy to traverse the WordReference
     * website. The Preserver object stores necessary state information about the
     * LanguageBuddy object. This includes saved words and (TODO)historical performance.
     * A Preserver allows the LanguageBuddy to maintain the st  
     */
    pub fn new(output_file: &str) -> anyhow::Result<LanguageBuddy>{
        let preserver = Preserver::read_from_file(output_file)?;

        let session = WordReferenceSpEnSession::new(WORD_REFERENCE_SP_EN_QUERY)?;

        let first_word = "pasar";

        session.lookup(first_word)?;

        Ok(
            LanguageBuddy{
                preserver: preserver,
                session: session,
                current_word: first_word.to_string(),
            }
        )
    }

    /** 
     * do_repl calls the loop. Also handles any necessary meta-data concerning loop lifetime
     */
    pub fn do_repl(&mut self) {
        loop {
            let result = self.repl();
            if result.is_err() {
                println!("encounted error: {}", result.unwrap_err());
            }
        } 
    }

    /**
     * repl is the main loop that handles user interaction.
     */
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
    }

    /** 
     * handle_word navigates the LanguageBuddy website and scrapes the page
     * contents for subsequent use.
     */
    fn handle_word(&mut self, word: &str) -> anyhow::Result<()>{
        let word = word.trim();
        if word.is_empty() {
            return Err(anyhow!("empty word"));
        }

        let result = self.session.navigate_and_scrape_page(word)?;
        println!("{:?}", result);

        Ok(())
    }

    /** 
     * handle_command runs a command supported by LanguageBuddy.
     */
    fn handle_command(&mut self, command: &Command) -> anyhow::Result<()>{
        match command {
            Command::Save => {
                match self.do_save(){
                    Ok(()) => println!("saved successfully!"),
                    Err(e) => println!("error: {}", e),
                }
            }
        }
        Ok(())
    }

    /** 
     * do_save saves the preserver result for later reuse. The LanguageBuddy
     * calls its preserver to save the given word.
     */
    fn do_save(&mut self) -> anyhow::Result<()>{
        self.preserver.add_string(&self.current_word);

        match self.preserver.write() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("failed to write word {} to preserver output: {}", self.current_word, e)),
        }
    }

    /** 
     * parse_input parses the user's input for later use.
     */
    fn parse_input(&self) -> anyhow::Result<UserInput> {
        io::stdout().flush()?;
        let word = input();

        if word.is_err() {
            return Err(anyhow!("problem receiving input: {}", word.unwrap_err()));
        };

        let word = word.unwrap();
        let word = word.trim();

        if word == "save" {
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
