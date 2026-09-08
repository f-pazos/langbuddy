use crate::content::FlashcardContent;

type CardID = String;

// struct FlashcardDeck {}
// impl FlashcardDeck {
//     fn get_cards() -> dyn FlashcardContent {}
// }



struct FlashcardRoutine {
    flashcards: Vec<Flashcard>,
    score_module: ScoreModule,
}


struct ScoreModule{...}


struct Flashcard {
    prompt: String,
    answer: String,
}

struct PromptState {
    active_flashcard: Flashcard
}
impl PromptState{
    fn initialize(display: DisplayHandle);

    /** 
     * Input should happen atomically; the terminal display handles communicating
     * input to the State.
     */
    fn handle_input(input: InputStream) -> TransitionSignal{
        case: 
            CONTROL_CHARACTER(C) -> return RoutineTransition(C);
            'R' -> Review,
            'D' -> DeckRefinement,
        
        case(t: TextInput) -> 
            if (&self.active_flashcard.is_correct(t)) -> HandleCorrect;
            else -> 
                HandleIncorrect
    };
}
