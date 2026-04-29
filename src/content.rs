pub trait FlashcardContent<I> {
    fn get_prompt(&self) -> &I;
    fn get_answer(&self) -> &I;
    fn compare_answer(guess: &I, answer: &I) -> bool;
}

pub struct StringContent {
    pub prompt: String,
    pub answer: String,
}

impl FlashcardContent<String> for StringContent {
    fn compare_answer(guess: &String, answer: &String) -> bool {
        guess == answer
    }
    fn get_answer(&self) -> &String {
        &self.answer
    }
    fn get_prompt(&self) -> &String {
        &self.prompt
    }
}
