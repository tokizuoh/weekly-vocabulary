pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Adjective,
    Verb,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
}

impl PartOfSpeech {
    pub fn text(&self) -> &str {
        match self {
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Pronoun => "pronoun",
            PartOfSpeech::Adjective => "adjectiv",
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Adverb => "adverb",
            PartOfSpeech::Preposition => "preposition",
            PartOfSpeech::Conjunction => "conjunction",
            PartOfSpeech::Interjection => "interjection",
        }
    }
}

pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary() -> String {
    // TODO: get from DB
    let vocabulary = Vocabulary {
        part_of_speech: PartOfSpeech::Verb,
        spelling: "transcribe".to_string(),
        meaning: "to write down something ecactly as it was said".to_string(),
    };

    vocabulary.spelling
}
