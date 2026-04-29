use std::fmt;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;

use crate::content;
use crate::flashcard;
use crate::flashcard::Flashcard;
use crate::preserver::Preserver;

enum REPLResult {
    Ok,
    SIGQuit,
    Err(anyhow::Error),
}

pub struct LanguageBuddy {
    preserver: Preserver,
    // session: browser_session::WordReferenceSpEnSession,
}

impl LanguageBuddy {
    /**
     * Returns a new LangBuddy instance.
     **/
    pub fn new() -> anyhow::Result<LanguageBuddy> {
        Ok(LanguageBuddy {
            preserver: Preserver::new(&fs::canonicalize(&PathBuf::from("./langbuddy_storage"))?)?,
        })
    }

    /**
     * do_repl calls the loop. Also handles any necessary meta-data concerning loop lifetime
     */
    pub fn do_repl(&mut self) -> anyhow::Result<()> {
        loop {
            let result = self.repl();
            match result {
                REPLResult::Err(e) => {
                    println!("error! {:?}", e);
                }
                REPLResult::Ok => continue,
                REPLResult::SIGQuit => return Ok(()),
            };
        }
        Ok(())
    }

    /**
     * repl is the main loop that handles user interaction.
     */
    pub fn repl(&mut self) -> REPLResult {
        let c = content::StringContent {
            prompt: "prompt?".to_string(),
            answer: "answer".to_string(),
        };
        let f = flashcard::VocabFlashcard::new(c);

        println!("{}", f.get_content().prompt);

        let mut buffer = String::new();
        let r = io::stdin().read_line(&mut buffer);
        match r {
            Err(e) => return REPLResult::Err(e.into()),
            Ok(_) => (),
        };

        let buffer = buffer.trim();

        if buffer == "quit" {
            return REPLResult::SIGQuit;
        };

        if buffer == f.get_content().answer {
            println!("omfg you fuckin got it chum, give him a point");
            self.preserver.add_line("one quarter portion....");
        }

        let x = self.preserver.write();
        if x.is_err() {
            return REPLResult::Err(x.unwrap_err());
        };

        REPLResult::Ok
    }

    /**
     * handle_word navigates the LanguageBuddy website and scrapes the page
     * contents for subsequent use.
     */
    fn handle_word(&mut self, word: &str) -> anyhow::Result<()> {
        // let word = word.trim();
        // if word.is_empty() {
        //     return Err(anyhow!("empty word"));
        // }

        // let result = self.session.navigate_and_scrape_page(word)?;
        // println!("{:?}", result);

        Ok(())
    }

    /**
     * handle_command runs a command supported by LanguageBuddy.
     */
    fn handle_command(&mut self, command: &Command) -> anyhow::Result<()> {
        match command {
            Command::Save => match self.do_save() {
                Ok(()) => println!("saved successfully!"),
                Err(e) => println!("error: {}", e),
            },
        }
        Ok(())
    }

    /**
     * do_save saves the preserver result for later reuse. The LanguageBuddy
     * calls its preserver to save the given word.
     */
    fn do_save(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

enum Command {
    Save,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Save => write!(f, "Save"),
        }
    }
}

enum UserInput {
    Command(Command),
    Word(String),
}
