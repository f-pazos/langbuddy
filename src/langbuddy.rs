use std::io;
use std::io::Write;

use crate::Preserver;
use crate::browser_session::{self, WordReferenceSpEnSession};

pub struct LanguageBuddy {
    preserver: Preserver,
    browser_session: browser_session::WordReferenceSpEnSession,
}

const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";
const WORD_REFERENCE_ENGLISH_GERMAN: &str = "https://www.wordreference.com/ende/";


impl LanguageBuddy {
    // Return a new LanguageBuddy instance.
    pub fn new(output_file: &str) -> anyhow::Result<LanguageBuddy>{
        let preserver = Preserver::read_from_file(output_file)?;

    let sp_en_session = WordReferenceSpEnSession::new(WORD_REFERENCE_SP_EN_QUERY)?;

    sp_en_session.lookup("pasar")?;
    if sp_en_session.get_definition().is_err(){
        println!("couldn't navigate to definition!");
    }



        Ok(
            LanguageBuddy{
                preserver: preserver,
                browser_session: sp_en_session,
            }
        )
    }

    pub fn repl(&mut self, sp_en_session: &browser_session::WordReferenceSpEnSession) -> anyhow::Result<()> { 
        let mut last_word = String::new();

        loop {
            io::stdout().flush()?;

            let word = input();
            if word.is_err() {
                println!("Problem with input.");
                continue;
            }
            let word = word.unwrap();

            if word == "\n" {
                if last_word.is_empty() || last_word == "\n" {
                    println!("no previous word to save");
                    continue;
                }
                let trimmed = last_word.trim();
                self.save(trimmed);

                last_word.clear();
                continue;
            }
            self.lookup_word(&word);
            last_word = word;
        }
    }

    fn save(&mut self, word: &str){
        self.preserver.add_string(word);
        match self.preserver.write() {
            Err(e) => println!("Error writing {} to preserver output: {}", word, e),
            Ok(_e) => println!("{} added to preserver output", word),
        }
    }

    fn lookup_word(&self, word: &str) {
        if self.browser_session.lookup(&word).is_err() {
            println!("Error looking up definition!");
        };

        let val = self.browser_session.get_definition();
        if val.is_err(){
            println!("Error getting definition: {}", val.unwrap_err());
        };
    }
}


fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}
