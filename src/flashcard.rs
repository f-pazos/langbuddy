use std::io;

use crate::{
    content::{self, FlashcardContent},
    flashcard,
    repl::REPLResult,
};

pub trait Flashcard<I, C: FlashcardContent<I>> {
    fn get_content(&self) -> &C;
    fn check_answer(&self, guess: I) -> bool {
        C::compare_answer(&guess, self.get_content().get_answer())
    }
}

pub struct VocabFlashcard {
    content: content::StringContent,
}

impl VocabFlashcard {
    pub fn new(content: content::StringContent) -> Self {
        Self { content }
    }
}

impl Flashcard<String, content::StringContent> for VocabFlashcard {
    fn get_content(&self) -> &content::StringContent {
        &self.content
    }
}

pub fn flashcard_routine() -> REPLResult {
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

    if f.check_answer(buffer.to_owned()) {
        println!("omfg you fuckin got it chum, give him a point");
        // self.preserver.add_line("one quarter portion....");
    } else {
        println!("lmao this guy sucks balls he's so bad");
    }

    // let x = self.preserver.write();
    // if x.is_err() {
    //     return REPLResult::Err(x.unwrap_err());
    // };

    REPLResult::Ok
}
