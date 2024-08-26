use std::io;
use std::io::Write;

use crate::Preserver;
use crate::browser_session;

struct LanguageBuddy {
    preserver: Preserver,
}

impl LanguageBuddy {
    // Return a new LanguageBuddy instance.
    fn new(output_file: &str) -> anyhow::Result<LanguageBuddy>{
        let preserver = Preserver::read_from_file(output_file)?;

        Ok(
            LanguageBuddy{
                preserver: preserver,
            }
        )
    }

    fn repl(&mut self, sp_en_session: &browser_session::WordReferenceSpEnSession) -> anyhow::Result<()> { 
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

                self.preserver.add_string(trimmed);
                match self.preserver.write() {
                    Err(e) => println!("Error writing {} to preserver output: {}", trimmed, e),
                    Ok(_e) => println!("{} added to preserver output", trimmed),
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
}

fn input() -> anyhow::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    return Ok(s);
}
