use std::iter::Map;

pub struct DictionaryNode {
    word: String,
    definitions: Vec<String>,
    homonyms: String,
}
pub struct Dictionary {
    // lexemes: Vec<Lexeme>,
    map: Map<Word, DictionaryEntry>,
}

// A word is a string with the added constraint that it
// is somehow meaningful in a language.
pub struct Word {
    token: String,
}

// A sentence is a list of words that is meaningful and
// grammatically correct.
pub struct Sentence {
    words: Vec<Word>,
}

enum PartOfSpeech {
    NOUN,
    VERB,
    ADJECTIVE,
}

// A definition represents the definition of a particular meaning of a word.
struct Definition {
    word: Word,
    part_of_speech: PartOfSpeech,
    definition: Sentence,
    examples: Vec<Sentence>,
}

// A dictionary entry represents an entry into a dictionary.
struct DictionaryEntry {
    word: Word,
    defitions: Vec<Definition>,
}