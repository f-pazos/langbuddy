use std::io;
use std::io::prelude::*;

mod preserver;
use preserver::Preserver;

mod browser_session;
use browser_session::WordReferenceSpEnSession;

// mod word;
mod parser;
mod langbuddy;

const OUTPUT_FILE: &str = "words.txt";

const WORD_REFERENCE: &str = "https://www.wordreference.com";
const WORD_REFERENCE_SP_EN_QUERY: &str =
    "https://www.wordreference.com/es/en/translation.asp?spen=";
const WORD_REFERENCE_ENGLISH_GERMAN: &str = "https://www.wordreference.com/ende/";

fn main() -> anyhow::Result<()> {
    let sp_en_session = WordReferenceSpEnSession::new(WORD_REFERENCE_SP_EN_QUERY)?;

    sp_en_session.lookup("pasar")?;
    if sp_en_session.get_definition().is_err(){
        println!("couldn't navigate to definition!");
    }

    repl(&sp_en_session)
}

fn repl(sp_en_session: &WordReferenceSpEnSession) -> anyhow::Result<()> { 
    let mut last_word = String::new();
    let mut preserver = Preserver::read_from_file(OUTPUT_FILE)?;

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

            preserver.add_string(trimmed);
            match preserver.write() {
                Err(e) => println!("Error writing {} to {}: {}", trimmed, OUTPUT_FILE, e),
                Ok(_e) => println!("{} added to {}", trimmed, OUTPUT_FILE),
            }

            last_word.clear();
            continue;
        }
        if sp_en_session.lookup(&word).is_err() {
            println!("Error looking up definition!");
        };

        let val = sp_en_session.get_definition();
        if val.is_err(){
            println!("Error getting definition: {}", val.unwrap_err());
        };

        last_word = word;
    }
}

fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}
