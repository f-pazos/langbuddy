use std::io;
use std::io::prelude::*;

mod preserver;
use preserver::Preserver;

mod browser_session;
use browser_session::WordReferenceSpEnSession;

// mod word;
mod parser;
mod langbuddy;
use langbuddy::LanguageBuddy;

const OUTPUT_FILE: &str = "/Users/fpazos/workspace/memorizer/words.txt";

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

    let mut lb = LanguageBuddy::new(OUTPUT_FILE)?;
    lb.repl(&sp_en_session)
}