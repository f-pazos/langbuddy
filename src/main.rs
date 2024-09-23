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

fn main() -> anyhow::Result<()> {
    let mut lb = LanguageBuddy::new(OUTPUT_FILE)?;
    lb.repl(&sp_en_session)
}