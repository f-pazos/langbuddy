use crate::content::{self, FlashcardContent};

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
