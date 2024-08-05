// pub struct FlashCard {
//     word: String,
//     definitions: String,
// }

// A dictionary has a vocabulary of words.
// A word is {representation, meaning}

// Some words have definition entries
// Some words have parts of speech

pub struct DictionaryNode {
    word: String,
    definitions: Vec<String>,
    homonyms: String,
}

// A dictionary is a static repository of information organized as a
// graph. 
// Each "word" has a definition 
// Each entry is a list of definitions
// each definition is a tuple of {POS, meaning: Sentence}
// each Sentence is itself a list of words, that each have entries associated
// with it. 
pub struct Dictionary {
    // lexemes: Vec<Lexeme>,
    pages: Vec<DictionaryPage>,
}

// A word has no meaning. It is token that, in a given lexical context (language),
// defines a set of possible lexemes it could be associated with.
pub struct Word {
    token: String,
}

pub struct Sentence {
    words: Vec<Word>,
}

enum PartOfSpeech {
    NOUN,
    VERB,
    ADJECTIVE,
}

struct Definition {
    word: Word,
    part_of_speech: PartOfSpeech,
    definition: Sentence,
    examples: Vec<Sentence>,
}
struct DictionaryEntry {
    word: Word,
    defitions: Vec<Definition>,
}