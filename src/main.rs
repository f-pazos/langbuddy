use std::io;
use std::io::prelude::*;

mod preserver;
use preserver::Preserver;

// mod browser_session;
// use browser_session::WordReferenceSpEnSession;

// // mod word;
// mod word_reference_scraper;
mod langbuddy;
use langbuddy::LanguageBuddy;


fn main() -> anyhow::Result<()> {
    let mut lb = LanguageBuddy::new()?;
    lb.do_repl()?;
    println!("success....");
    Ok(())
}