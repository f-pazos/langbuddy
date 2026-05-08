use std::io;
use std::io::prelude::*;

// mod filesystem;
// use filesystem::FileBackedBuffer;

// mod browser_session;
// use browser_session::WordReferenceSpEnSession;

// // mod word;
// mod word_reference_scraper;
mod repl;
use repl::TopLevelREPL;

mod constants;
// mod content;
// mod flashcard;
// mod flashcard_deck;

fn main() -> anyhow::Result<()> {
    let mut repl = TopLevelREPL::new()?;
    repl.do_repl()?;
    println!("success....");
    Ok(())
}
